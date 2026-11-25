extends Node

signal changed
signal finished(entry: Dictionary)
var running := false
var cancelled := false
var rows: Array = []
var message := ""
var started := 0
var options: Dictionary
var _queue: Array = []
var _active := 0
var _paths := {}
var _token := ""
var _last_progress := 0


func start(repositories: Array, config: Dictionary, token: String):
	if running or repositories.is_empty():
		return
	options = config.duplicate(true)
	_token = token
	running = true
	cancelled = false
	started = Time.get_ticks_msec()
	message = ""
	rows.clear()
	_paths.clear()
	_queue.clear()
	for repo in repositories:
		var row = {"repo": repo, "status": "Queued", "error": ""}
		var dest = options.destination
		if options.splitBy == "owner":
			dest = dest.path_join(repo.owner.login)
		row.path = dest.path_join(repo.name)
		rows.append(row)
		if _paths.has(row.path.to_lower()):
			row.status = "Failed"
			row.error = "Duplicate destination. Group by owner or select one repository."
		else:
			_paths[row.path.to_lower()] = true
			_queue.append(row)
	while not _queue.is_empty() or _active > 0:
		while not _queue.is_empty() and _active < clampi(int(options.maxConcurrent), 1, 8):
			var row = _queue.pop_front()
			if cancelled:
				row.status = "Cancelled"
			else:
				_run_repo(row)
		changed.emit()
		await get_tree().create_timer(0.15).timeout
	_token = ""
	var failed := []
	var done := 0
	for row in rows:
		if row.status == "Complete":
			done += 1
		if row.status == "Failed":
			failed.append(row.repo.full_name)
	var archive_error := ""
	if options.createZip and done > 0 and not cancelled:
		var filename = (
			"%s-backup-%s.zip"
			% [repositories[0].owner.login, Time.get_date_string_from_system(true)]
		)
		archive_error = await _archive(options.destination, filename)
	var status := "complete"
	if cancelled:
		status = "cancelled"
	elif done == 0:
		status = "failed"
	elif not failed.is_empty() or not archive_error.is_empty():
		status = "partial"
	message = archive_error if not archive_error.is_empty() else "Backup " + status
	running = false
	var entry = {
		"id": str(int(Time.get_unix_time_from_system() * 1000)),
		"date": Time.get_datetime_string_from_system(true) + "Z",
		"repoCount": rows.size(),
		"totalSize": 0,
		"duration": Time.get_ticks_msec() - started,
		"destination": options.destination,
		"status": status,
		"failedRepos": failed,
		"options": options.duplicate(true)
	}
	finished.emit(entry)
	changed.emit()


func _run_repo(row: Dictionary):
	_active += 1
	row.status = "Cloning"
	var path: String = row.path
	var url: String = row.repo.clone_url
	var problem := ""
	var mirror: bool = options.cloneType == "mirror"
	if DirAccess.make_dir_recursive_absolute(path.get_base_dir()) != OK:
		problem = "Cannot create destination."
	elif DirAccess.dir_exists_absolute(path):
		var origin = await _git(["-C", path, "config", "--local", "--get", "remote.origin.url"])
		var bare = await _git(["-C", path, "rev-parse", "--is-bare-repository"])
		if (
			origin.code != 0
			or origin.text.strip_edges() != url
			or bare.text.strip_edges() != ("true" if mirror else "false")
		):
			problem = "The existing folder is not a matching clone."
		else:
			row.status = "Updating"
			var args = (
				["-C", path, "remote", "update", "--prune"]
				if mirror
				else ["-C", path, "pull", "--ff-only"]
			)
			problem = _git_problem(await _git(args))
	else:
		var stage = path + ".partial-" + Crypto.new().generate_random_bytes(8).hex_encode()
		var args = ["clone"]
		if mirror:
			args.append("--mirror")
		args.append_array(["--", url, stage])
		problem = _git_problem(await _git(args))
		if problem.is_empty() and DirAccess.rename_absolute(stage, path) != OK:
			problem = "Cannot move completed clone into place."
		if not problem.is_empty() and DirAccess.dir_exists_absolute(stage):
			_remove_stage(stage)
	row.error = problem
	row.status = "Complete" if problem.is_empty() else "Failed"
	_active -= 1
	changed.emit()


func _git_problem(result: Dictionary) -> String:
	return (
		""
		if result.code == 0
		else ("Git exited with code %d. %s" % [result.code, result.text.strip_edges().right(350)])
	)


func _git(args: Array) -> Dictionary:
	var cmd = ["-c", "credential.helper=", "-c", "http.followRedirects=false"]
	# Git inherits the scoped header at spawn. Keep credentials out of command
	# arguments, remote URLs and the clone's persistent configuration.
	var had_auth := OS.has_environment("GOSH_GIT_AUTH")
	var previous_auth := OS.get_environment("GOSH_GIT_AUTH")
	var secret := ""
	if not _token.is_empty():
		secret = Marshalls.utf8_to_base64("x-access-token:" + _token)
		cmd.append("--config-env=http.https://github.com/.extraHeader=GOSH_GIT_AUTH")
		OS.set_environment("GOSH_GIT_AUTH", "Authorization: Basic " + secret)
	cmd.append_array(args)
	var prev := OS.get_environment("GIT_TERMINAL_PROMPT")
	var had := OS.has_environment("GIT_TERMINAL_PROMPT")
	OS.set_environment("GIT_TERMINAL_PROMPT", "0")
	var proc := OS.execute_with_pipe("git", PackedStringArray(cmd), false)
	if had:
		OS.set_environment("GIT_TERMINAL_PROMPT", prev)
	else:
		OS.unset_environment("GIT_TERMINAL_PROMPT")
	if not secret.is_empty():
		if had_auth:
			OS.set_environment("GOSH_GIT_AUTH", previous_auth)
		else:
			OS.unset_environment("GOSH_GIT_AUTH")
	if proc.is_empty():
		return {"code": -1, "text": "Install Git and ensure it is on PATH."}
	var output := ""
	while OS.is_process_running(proc.pid):
		for stream in [proc.stdio, proc.stderr]:
			output = (output + stream.get_buffer(16384).get_string_from_utf8()).right(4096)
		await get_tree().create_timer(0.05).timeout
	for stream in [proc.stdio, proc.stderr]:
		output += stream.get_buffer(16384).get_string_from_utf8()
	if not secret.is_empty():
		output = output.replace(secret, "[redacted]").replace(_token, "[redacted]")
	return {"code": OS.get_process_exit_code(proc.pid), "text": output}


func _remove_stage(path: String):
	var dir := DirAccess.open(path)
	if dir == null:
		return
	dir.include_hidden = true
	for name in dir.get_files():
		dir.remove(name)
	for name in dir.get_directories():
		if dir.is_link(name):
			dir.remove(name)
		else:
			_remove_stage(path.path_join(name))
	DirAccess.remove_absolute(path)


func _archive(root: String, filename: String) -> String:
	var zip := ZIPPacker.new()
	var temp := root.path_join(
		filename + ".partial-" + Crypto.new().generate_random_bytes(8).hex_encode()
	)
	if zip.open(temp) != OK:
		return "Cannot open ZIP at the destination."
	zip.compression_level = clampi(int(options.zipCompression), 0, 9)
	message = "Creating ZIP"
	changed.emit()
	var dirs := [""]
	var files := []
	var error := OK
	while not dirs.is_empty() and error == OK and not cancelled:
		var rel: String = dirs.pop_back()
		var dir := DirAccess.open(root.path_join(rel))
		if not dir:
			error = ERR_CANT_OPEN
			break
		dir.include_hidden = true
		dir.list_dir_begin()
		var name := dir.get_next()
		while name != "" and error == OK and not cancelled:
			var child := rel.path_join(name)
			var path := root.path_join(child)
			var skip := dir.is_link(name) or path == temp or child == filename
			for pattern in options.excludePatterns:
				if name.match(pattern):
					skip = true
			if not skip:
				if dir.current_is_dir():
					error = zip.add_directory(child)
					dirs.append(child)
				else:
					files.append(child)
			if Time.get_ticks_msec() - _last_progress > 40:
				message = "Preparing ZIP · %d files" % files.size()
				changed.emit()
				await get_tree().process_frame
				_last_progress = Time.get_ticks_msec()
			name = dir.get_next()
		dir.list_dir_end()
	for i in files.size():
		if error != OK or cancelled:
			break
		var child: String = files[i]
		message = "Archiving %d / %d · %s" % [i + 1, files.size(), child]
		error = await _zip_file(zip, root.path_join(child), child)
	var close_error := zip.close()
	if cancelled or error != OK or close_error != OK:
		DirAccess.remove_absolute(temp)
		return "Archive cancelled." if cancelled else "Failed to write ZIP."
	if DirAccess.rename_absolute(temp, root.path_join(filename)) != OK:
		DirAccess.remove_absolute(temp)
		return "Cannot replace destination ZIP."
	return ""


func _zip_file(zip: ZIPPacker, path: String, rel: String) -> Error:
	var file := FileAccess.open(path, FileAccess.READ)
	if not file:
		return FileAccess.get_open_error()
	var error := zip.start_file(rel)
	if error != OK:
		return error
	if Time.get_ticks_msec() - _last_progress > 40:
		changed.emit()
		await get_tree().process_frame
		_last_progress = Time.get_ticks_msec()
	while file.get_position() < file.get_length() and not cancelled and error == OK:
		var data := file.get_buffer(mini(1048576, file.get_length() - file.get_position()))
		if data.is_empty():
			return ERR_FILE_CANT_READ
		error = zip.write_file(data)
		if Time.get_ticks_msec() - _last_progress > 40:
			changed.emit()
			await get_tree().process_frame
			_last_progress = Time.get_ticks_msec()
	var end_error := zip.close_file()
	return error if error != OK else end_error
