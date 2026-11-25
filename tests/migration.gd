extends SceneTree

const Store = preload("res://store.gd")
const MainScene = preload("res://main.tscn")
var checks := 0
var failures := 0
var summary := {}
var fixture: String
var app


func _initialize():
	call_deferred("_test")


func expect(condition: bool, detail: String):
	checks += 1
	if not condition:
		failures += 1
		push_error(detail)


func _test():
	fixture = OS.get_environment("GOSH_TEST_ROOT")
	if (
		fixture.is_empty()
		or OS.get_user_data_dir().get_file() != fixture.get_file()
		or not fixture.get_file().begins_with("gosh-migration-")
	):
		printerr("Run tests/migration.sh to isolate user data.")
		quit(2)
		return
	var marker := FileAccess.open(fixture.path_join("profile-path"), FileAccess.WRITE)
	marker.store_string(OS.get_user_data_dir())
	marker.close()
	var repo = {
		"id": 1,
		"name": "example",
		"full_name": "fixture/example",
		"owner": {"login": "fixture"},
		"clone_url": fixture.path_join("origin"),
		"visibility": "private",
		"private": true,
		"fork": false,
		"language": "GDScript",
		"description": "fixture",
		"archived": false,
		"stargazers_count": 7,
		"size": 4,
		"updated_at": "2026-01-01",
		"pushed_at": Time.get_datetime_string_from_system(true)
	}
	var legacy := FileAccess.open(Store.PATH, FileAccess.WRITE)
	legacy.store_string(
		JSON.stringify(
			{
				"token": null,
				"settings":
				{
					"theme": "dark",
					"notifications": false,
					"defaultBackupLocation": fixture,
					"defaultBackupOptions": {"maxConcurrent": 2}
				},
				"backup_history":
				[
					{
						"id": "legacy-run",
						"date": "2026-01-01T00:00:00Z",
						"repoCount": 1,
						"totalSize": 12,
						"duration": 1000,
						"destination": fixture,
						"status": "complete",
						"failedRepos": [],
						"options": {}
					}
				]
			}
		)
	)
	legacy.close()
	app = MainScene.instantiate()
	root.add_child(app)
	expect(
		(
			not app.store.settings.notifications
			and app.options.maxConcurrent == 2
			and app.options.destination == fixture
		),
		"Legacy profile loaded"
	)
	var endpoint = (
		"http://127.0.0.1:" + FileAccess.get_file_as_string(fixture.path_join("port")).strip_edges()
	)
	app.hub.api_root = endpoint
	app.hub.device_root = endpoint
	await app.login("invalid-fixture")
	expect(
		app.page == "Connect" and app.feedback.contains("401") and Store.new().data.token == null,
		"Rejected PAT is not persisted"
	)
	await app.login("fixture-session")
	expect(
		(
			app.page == "Dashboard"
			and app.account.repos.size() == 101
			and Store.new().data.token == "fixture-session"
		),
		"PAT, pagination and token storage"
	)
	var first = app
	app = MainScene.instantiate()
	app.hub.api_root = endpoint
	app.hub.device_root = endpoint
	root.add_child(app)
	while app.busy:
		await process_frame
	expect(
		app.page == "Dashboard" and app.account.repos.size() == 101,
		"Legacy profile resumes a signed-in session"
	)
	first.queue_free()
	app._logout()
	app._change_method("Device sign-in")
	await app._oauth()
	while app.busy:
		await process_frame
	expect(
		app.page == "Dashboard" and app.account.user.login == "fixture",
		"OAuth device sign-in and pending poll"
	)
	app._logout()
	app._change_method("Device sign-in")
	app._change_method("Token")
	app.account = {"user": {"login": "fixture"}, "repos": [repo], "orgs": [], "events": []}
	for page in ["Dashboard", "Repositories", "Settings", "About", "Backup"]:
		app.go(page)
	app.go("Repositories")
	app._select_many(true)
	expect(app.selected.size() == 1, "Recent repository selection")
	app._filter_changed("Public", "Visibility")
	expect(app._filtered().is_empty(), "Private repositories hidden by public filter")
	app._filter_changed("All", "Visibility")
	app._filter_changed("Python", "Language")
	expect(app._filtered().is_empty(), "Language filtering")
	app._filter_changed("All", "Language")
	app._filter_changed("another-owner", "Owner")
	expect(app._filtered().is_empty(), "Owner filtering")
	app._filter_changed("All", "Owner")
	app.query = "FIXTURE/EXAMPLE"
	expect(app._filtered().size() == 1, "Case-insensitive repository search")
	app.query = ""
	for mode in ["light", "dark", "system"]:
		app.store.settings.theme = mode
		app.persist()
		app.go("Settings")
		expect(Store.new().settings.theme == mode, "Theme retained: " + mode)
	app.backup._token = "fixture-session"
	var header = await app.backup._git(
		["config", "--get-urlmatch", "http.extraHeader", "https://github.com/fixture/example"]
	)
	expect(
		header.code == 0 and header.text.contains("Basic [redacted]"),
		"Git receives the scoped authentication header"
	)
	var unrelated = await app.backup._git(
		["config", "--get-urlmatch", "http.extraHeader", "https://example.invalid/fixture/example"]
	)
	expect(
		not unrelated.text.contains("Basic [redacted]"),
		"GitHub authentication stays scoped to GitHub"
	)
	app.backup._token = ""

	app._set_tab("Options")
	app.options.destination = fixture.path_join("backup")
	repo.archived = true
	app._start()
	expect(
		not app.backup.running and app.feedback.contains("backup options"),
		"Archive inclusion policy"
	)
	repo.archived = false
	repo.fork = true
	app.options.includeForks = false
	app._start()
	expect(not app.backup.running, "Fork inclusion policy")
	repo.fork = false
	app.options.includeForks = true

	app.options.createZip = true
	app.options.excludePatterns = [".git", "*.tmp"]
	app.backup.finished.connect(func(entry): summary = entry)
	app._start()
	await app.backup.finished
	expect(summary.status == "complete", "Full backup: " + str(app.backup.rows))
	var clone: String = app.options.destination.path_join("fixture/example")
	expect(
		FileAccess.get_file_as_string(clone.path_join("example.txt")) == "original\n",
		"Cloned file content"
	)
	var zip_path: String = app.options.destination.path_join(
		"fixture-backup-" + Time.get_date_string_from_system(true) + ".zip"
	)
	var zip := ZIPReader.new()
	expect(zip.open(zip_path) == OK, "ZIP opens")
	if zip.file_exists("fixture/example/example.txt"):
		expect(
			zip.read_file("fixture/example/example.txt").get_string_from_utf8() == "original\n",
			"Archive content"
		)
	else:
		expect(false, "Archive missing expected entry")
	expect(not zip.file_exists("fixture/example/skip.tmp"), "Archive excludes")
	zip.close()
	var f := FileAccess.open(fixture.path_join("origin/example.txt"), FileAccess.WRITE)
	f.store_string("updated\n")
	f.close()
	expect(
		OS.execute("git", ["-C", fixture.path_join("origin"), "commit", "-qam", "update"]) == 0,
		"Update fixture"
	)
	await app.backup.start([repo], app.options, "")
	expect(
		(
			summary.status == "complete"
			and FileAccess.get_file_as_string(clone.path_join("example.txt")) == "updated\n"
		),
		"Existing clone updates"
	)
	app.options.cloneType = "mirror"
	app.options.destination = fixture.path_join("mirrors")
	app.options.createZip = false
	await app.backup.start([repo], app.options, "")
	expect(
		(
			summary.status == "complete"
			and FileAccess.file_exists(app.options.destination.path_join("fixture/example/HEAD"))
		),
		"Mirror clone"
	)
	await app.backup.start([repo], app.options, "")
	expect(summary.status == "complete", "Existing mirror updates")
	var broken = repo.duplicate(true)
	broken.name = "missing"
	broken.clone_url = fixture.path_join("absent")
	await app.backup.start([broken, repo], app.options, "")
	expect(
		summary.status == "partial" and summary.failedRepos.size() == 1,
		"Concurrent partial failure"
	)
	app._set_tab("History")
	var reopened = Store.new()
	expect(
		(
			reopened.data.backup_history.size() == 6
			and reopened.data.backup_history.back().id == "legacy-run"
		),
		"History persists"
	)
	if OS.get_name() != "Windows":
		expect(FileAccess.get_unix_permissions(Store.PATH) == 384, "Unix profile permissions")
	var second = repo.duplicate(true)
	second.name = "second"
	second.full_name = "fixture/second"
	app.options.maxConcurrent = 1
	app.backup.start([repo, second], app.options, "")
	app.backup.cancelled = true
	await app.backup.finished
	expect(
		summary.status == "cancelled" and app.backup.rows[1].status == "Cancelled",
		"Cancel leaves queued repositories untouched"
	)
	var occupied = fixture.path_join("occupied")
	DirAccess.make_dir_recursive_absolute(occupied.path_join("example"))
	var marker_file := FileAccess.open(occupied.path_join("example/keep.txt"), FileAccess.WRITE)
	marker_file.store_string("keep")
	marker_file.close()
	app.options.destination = occupied
	app.options.splitBy = "none"
	await app.backup.start([repo], app.options, "")
	expect(
		(
			summary.status == "failed"
			and FileAccess.get_file_as_string(occupied.path_join("example/keep.txt")) == "keep"
		),
		"Existing unrelated folders are preserved"
	)
	app._clear_history()
	app._logout()
	expect(
		app.account.is_empty() and Store.new().data.backup_history.is_empty(),
		"Clear history and logout"
	)
	print("Checks executed: ", checks)
	print("Migration checks complete. Failures: ", failures)
	quit(1 if failures else 0)
