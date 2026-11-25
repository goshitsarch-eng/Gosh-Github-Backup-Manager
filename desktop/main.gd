extends Control

const Store = preload("store.gd")
const Hub = preload("github.gd")
const Backup = preload("backup.gd")
const PROJECT = "https://github.com/goshitsarch-eng/Gosh-Github-Backup-Manager"
var store = Store.new()
var hub = Hub.new()
var backup = Backup.new()
const DOCS = "https://docs.goshapps.com/7/gosh-github-backup-manager"
var account := {}
var selected := {}
var options: Dictionary
var page := "Connect"
var tab := "Options"
var auth_method := "Token"
var auth_generation := 0
var busy := false
var quit_after_backup := false
var device_expiry := 0
var expiry_label: Label
var device := {}
var feedback := ""
var query := ""
var filters := {"Owner": "All", "Visibility": "All", "Language": "All", "Sort": "Updated"}
var descending := true
var canvas: ColorRect
var content: VBoxContainer
var repo_list: Tree
var selection_label: Label
var progress_list: Tree
var progress_bar: ProgressBar
var progress_label: Label
var ink: Color
var muted: Color
var accent: Color
var panel: Color
var ground: Color
var edge: Color


func _ready():
	get_tree().auto_accept_quit = false
	get_window().close_requested.connect(_close_requested)
	get_window().min_size = Vector2i(800, 600)
	add_child(hub)
	add_child(backup)
	backup.finished.connect(_finished)
	backup.changed.connect(_update_progress)
	options = store.settings.defaultBackupOptions.duplicate(true)
	options.destination = store.settings.defaultBackupLocation
	feedback = store.problem
	draw_page()
	if store.data.token is String and not store.data.token.is_empty():
		login(store.data.token)


func box(color: Color, radius := 9, pad := 12) -> StyleBoxFlat:
	var b := StyleBoxFlat.new()
	b.bg_color = color
	b.set_corner_radius_all(radius)
	b.set_content_margin_all(pad)
	return b


func _process(_delta):
	if not device.is_empty() and is_instance_valid(expiry_label):
		var seconds := maxi(0, ceili((device_expiry - Time.get_ticks_msec()) / 1000.0))
		expiry_label.text = "Expires in %d:%02d" % [floori(seconds / 60.0), seconds % 60]


func colors():
	var light: bool = (
		store.settings.theme == "light"
		or (
			store.settings.theme == "system"
			and DisplayServer.is_dark_mode_supported()
			and not DisplayServer.is_dark_mode()
		)
	)
	ground = Color("#eef2f2") if light else Color("#101b20")
	panel = Color("#ffffff") if light else Color("#19282e")
	ink = Color("#15282b") if light else Color("#e8f1ee")
	muted = Color("#50696a") if light else Color("#b1c4c2")
	accent = Color("#076a5c") if light else Color("#8fe2cc")
	edge = Color("#d8e6e2") if light else Color("#31454a")
	var t := Theme.new()
	t.default_font_size = 16
	var font := SystemFont.new()
	font.font_names = PackedStringArray(["Inter", "Segoe UI", "Helvetica Neue", "DejaVu Sans"])
	t.default_font = font
	for type in ["Label", "Button", "OptionButton", "CheckBox", "LineEdit", "Tree", "SpinBox"]:
		t.set_color("font_color", type, ink)
		t.set_color("font_hover_color", type, ink)
		t.set_color("font_pressed_color", type, ink)
		t.set_color("font_selected_color", type, ink)
	for type in ["Button", "OptionButton", "LineEdit", "Tree"]:
		t.set_stylebox("normal" if type != "Tree" else "panel", type, box(panel))
		t.set_stylebox("hover", type, box(edge))
		t.set_stylebox("pressed", type, box(edge))
	t.set_stylebox("panel", "PanelContainer", box(panel, 14, 20))
	var input_style := box(ground)
	input_style.set_border_width_all(1)
	input_style.border_color = edge
	t.set_stylebox("normal", "LineEdit", input_style)
	t.set_color("font_placeholder_color", "LineEdit", muted)
	t.set_color("font_uneditable_color", "LineEdit", muted)
	t.set_color("font_outline_color", "ProgressBar", ground)
	t.set_stylebox("background", "ProgressBar", box(edge, 4, 2))
	t.set_stylebox("fill", "ProgressBar", box(accent, 4, 2))
	t.set_constant("v_separation", "Tree", 10)
	t.set_stylebox("selected", "Tree", box(edge, 4, 2))
	t.set_stylebox("selected_focus", "Tree", box(edge, 4, 2))
	theme = t


func column(parent: Node, gap := 12) -> VBoxContainer:
	var node := VBoxContainer.new()
	node.add_theme_constant_override("separation", gap)
	parent.add_child(node)
	return node


func row(parent: Node, gap := 10) -> HBoxContainer:
	var node := HBoxContainer.new()
	node.add_theme_constant_override("separation", gap)
	parent.add_child(node)
	return node


func label(parent: Node, value: String, size := 16, color := Color.WHITE) -> Label:
	var item := Label.new()
	item.text = value
	item.add_theme_font_size_override("font_size", size)
	item.add_theme_color_override("font_color", ink if color == Color.WHITE else color)
	parent.add_child(item)
	return item


func button(parent: Node, title: String, action: Callable, primary := false) -> Button:
	var b := Button.new()
	b.text = title
	b.mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
	if primary:
		b.add_theme_stylebox_override("normal", box(accent))
		b.add_theme_color_override("font_color", ground)
	b.pressed.connect(action)
	parent.add_child(b)
	return b


func fill(node: Control):
	node.size_flags_horizontal = Control.SIZE_EXPAND_FILL


func field(parent: Node, value: String, hint: String) -> LineEdit:
	var item := LineEdit.new()
	item.text = value
	item.placeholder_text = hint
	fill(item)
	parent.add_child(item)
	return item


func pick(parent: Node, items: Array, current: String, action: Callable) -> OptionButton:
	var item := OptionButton.new()
	for value in items:
		item.add_item(value)
	item.select(maxi(0, items.find(current)))
	item.item_selected.connect(func(index): action.call(items[index]))
	parent.add_child(item)
	return item


func check(parent: Node, title: String, value: bool, action: Callable):
	var item := CheckBox.new()
	item.text = title
	item.button_pressed = value
	item.toggled.connect(action)
	parent.add_child(item)


func card(parent: Node) -> VBoxContainer:
	var frame := PanelContainer.new()
	parent.add_child(frame)
	return column(frame)


func go(where: String):
	page = where
	feedback = ""
	draw_page()


func draw_page():
	if is_instance_valid(canvas):
		remove_child(canvas)
		canvas.queue_free()
	colors()
	canvas = ColorRect.new()
	canvas.color = ground
	add_child(canvas)
	canvas.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
	var layout := row(canvas, 0)
	layout.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
	if not account.is_empty():
		var side_panel := PanelContainer.new()
		layout.add_child(side_panel)
		side_panel.custom_minimum_size.x = 175
		var side := column(side_panel, 8)
		label(side, "Gosh", 32, accent)
		label(side, "Backup Manager", 15, muted)
		side.add_child(HSeparator.new())
		for dest in ["Dashboard", "Repositories", "Backup", "Settings", "About"]:
			button(side, dest, go.bind(dest), page == dest)
		button(side, "User manual", OS.shell_open.bind(DOCS))
		var spacer := Control.new()
		spacer.size_flags_vertical = Control.SIZE_EXPAND_FILL
		side.add_child(spacer)
		label(side, "@" + str(account.user.login), 14, muted)
		button(side, "Disconnect", _confirm.bind("Disconnect account?", "This removes the saved token from this computer.", _logout)).disabled = (
			backup.running
		)
	var scroll := ScrollContainer.new()
	fill(scroll)
	layout.add_child(scroll)
	var margin := MarginContainer.new()
	fill(margin)
	for direction in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + direction, 28)
	if page == "Connect":
		var center := CenterContainer.new()
		fill(center)
		center.size_flags_vertical = Control.SIZE_EXPAND_FILL
		scroll.add_child(center)
		margin.custom_minimum_size.x = 640
		center.add_child(margin)
	else:
		scroll.add_child(margin)
	content = column(margin, 16)
	fill(content)
	if not feedback.is_empty():
		var msg = label(content, feedback, 15, accent)
		msg.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	match page:
		"Connect":
			_auth_page()
		"Dashboard":
			_dashboard()
		"Repositories":
			_repositories()
		"Backup":
			_backup_page()
		"Settings":
			_settings()
		"About":
			_about()


func _auth_page():
	var brand := row(content, 12)
	var icon := TextureRect.new()
	icon.texture = load("res://icon.png")
	icon.custom_minimum_size = Vector2(40, 40)
	icon.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
	icon.stretch_mode = TextureRect.STRETCH_KEEP_ASPECT_CENTERED
	brand.add_child(icon)
	label(brand, "Gosh  /  Backup Manager", 20, accent)
	label(content, "Connect to GitHub", 40)
	label(content, "Sign in to choose repositories for a local backup.", 17, muted)
	var section := card(content)
	var tabs := row(section)
	for method in ["Token", "Device sign-in"]:
		fill(button(tabs, method, _change_method.bind(method), auth_method == method))
	if auth_method == "Token":
		label(section, "Personal access token", 19)
		var input := field(section, "", "Enter your GitHub token")
		input.secret = true
		input.text_submitted.connect(login)
		var connect_button := button(
			section, "Connecting…" if busy else "Connect", func(): login(input.text), true
		)
		connect_button.disabled = true
		input.text_changed.connect(
			func(value): connect_button.disabled = busy or value.strip_edges().is_empty()
		)
		label(section, "Required scopes   repo · read:user · read:org", 14, muted)
		button(
			section,
			"Create a token on GitHub",
			OS.shell_open.bind(
				"https://github.com/settings/tokens/new?scopes=repo,read:user,read:org"
			)
		)
	elif device.is_empty():
		label(section, "Authorize this device in your browser.", 18)
		button(section, "Connecting…" if busy else "Sign in with GitHub", _oauth, true).disabled = busy
	else:
		label(section, str(device.user_code), 32, accent)
		expiry_label = label(section, "", 15, muted)
		button(section, "Copy device code", DisplayServer.clipboard_set.bind(str(device.user_code)))
		button(
			section,
			"Open GitHub verification",
			OS.shell_open.bind(device.get("verification_uri", "https://github.com/login/device")),
			true
		)
		button(section, "Cancel", _change_method.bind(auth_method))


func _change_method(method: String):
	auth_generation += 1
	busy = false
	device = {}
	auth_method = method
	go("Connect")


func login(raw: String):
	var token := raw.strip_edges()
	if token.is_empty() or busy:
		return
	busy = true
	auth_generation += 1
	var generation := auth_generation
	feedback = "Loading your GitHub account…"
	draw_page()
	hub.token = token
	var result = await hub.account()
	if generation != auth_generation:
		return
	busy = false
	if result.has("error"):
		if account.is_empty():
			hub.token = ""
		feedback = result.error
		draw_page()
		return
	if not account.is_empty() and account.user.login == result.user.login:
		for key in result.unavailable:
			result[key] = account[key]
	account = result
	var available := {}
	for repo in account.repos:
		available[str(repo.id)] = true
	for id in selected.keys():
		if not available.has(id):
			selected.erase(id)
	store.data.token = token
	page = "Dashboard"
	feedback = "\n".join(result.warnings)
	persist()
	draw_page()


func _oauth():
	if busy:
		return
	busy = true
	auth_generation += 1
	var gen := auth_generation
	draw_page()
	var res = await hub.request_json(
		hub.device_root + "/login/device/code",
		{"client_id": Hub.CLIENT, "scope": "repo,read:user,read:org"}
	)
	if gen != auth_generation:
		return
	if (
		res.has("error")
		or not res.value is Dictionary
		or not res.value.has_all(["user_code", "device_code", "expires_in"])
	):
		_auth_failure(res.get("error", "Device authorization failed."))
		return
	device = res.value
	var interval := maxi(1, int(device.get("interval", 5)))
	var deadline := Time.get_ticks_msec() + int(device.expires_in) * 1000
	device_expiry = deadline
	draw_page()
	while Time.get_ticks_msec() < deadline:
		await get_tree().create_timer(interval).timeout
		if gen != auth_generation:
			return
		if Time.get_ticks_msec() >= deadline:
			break
		res = await hub.request_json(
			hub.device_root + "/login/oauth/access_token",
			{
				"client_id": Hub.CLIENT,
				"device_code": device.device_code,
				"grant_type": "urn:ietf:params:oauth:grant-type:device_code"
			}
		)
		if gen != auth_generation:
			return
		if res.has("error"):
			_auth_failure(res.error)
			return
		if not res.value is Dictionary:
			_auth_failure("Unexpected response during device sign-in.")
			return
		if res.value.has("access_token"):
			busy = false
			device = {}
			login(res.value.access_token)
			return
		match res.value.get("error", ""):
			"slow_down":
				interval += 5
			"authorization_pending":
				pass
			_:
				_auth_failure(res.value.get("error_description", "Device authorization ended."))
				return
	_auth_failure("Device code expired. Request a new code.")


func _auth_failure(reason: String):
	device = {}
	busy = false
	feedback = reason
	draw_page()


func persist():
	var error = store.save()
	if error != OK:
		feedback = (
			store.problem
			if not store.problem.is_empty()
			else "Could not save preferences: " + error_string(error)
		)


func _dashboard():
	label(content, "Dashboard", 36)
	var latest: Array = store.data.backup_history
	label(
		content,
		(
			"Last backup   "
			+ ("Never" if latest.is_empty() else str(latest[0].date).get_slice("T", 0))
		),
		16,
		muted
	)
	var stats := row(content)
	var stars := 0
	var forks := 0
	for repo in account.repos:
		stars += int(repo.stargazers_count)
		forks += int(repo.get("forks_count", 0))
	for pair in [["Repositories", account.repos.size()], ["Stars", stars], ["Forks", forks]]:
		var tile := card(stats)
		fill(tile.get_parent())
		label(tile, str(pair[1]), 28, accent)
		label(tile, pair[0], 15, muted)
	var tools := row(content)
	button(tools, "Choose repositories", go.bind("Repositories"), true)
	button(tools, "Refresh account", login.bind(hub.token)).disabled = busy
	var recent := row(content)
	fill(label(recent, "Recent backups", 23))
	button(recent, "View history", _set_tab.bind("History"))
	_history(5)
	label(content, "GitHub activity", 23)
	for event in account.events:
		label(content, "%s   ·   %s" % [event.type, event.repo.name], 15, muted)
	if account.events.is_empty():
		label(content, "No recent activity.", 15, muted)


func _filtered() -> Array:
	var result := []
	for repo in account.repos:
		var language = repo.language if repo.language else "Unknown"
		if filters.Owner != "All" and filters.Owner != repo.owner.login:
			continue
		if filters.Language != "All" and filters.Language != language:
			continue
		if filters.Visibility != "All" and (filters.Visibility == "Private") != bool(repo.private):
			continue
		if (
			not query.is_empty()
			and not (str(repo.full_name) + " " + str(repo.description)).to_lower().contains(
				query.to_lower()
			)
		):
			continue
		result.append(repo)
	var key: String = {
		"Updated": "updated_at", "Stars": "stargazers_count", "Size": "size", "Name": "name"
	}[filters.Sort]
	result.sort_custom(
		func(a, b):
			var left = a[key].to_lower() if key == "name" else a[key]
			var right = b[key].to_lower() if key == "name" else b[key]
			return left > right if descending else left < right
	)
	return result


func _repositories():
	label(content, "Repositories", 36)
	var search := field(content, query, "Search by name or description")
	search.text_changed.connect(
		func(s):
			query = s
			_populate_repos()
	)
	var bar := HFlowContainer.new()
	content.add_child(bar)
	var owners := []
	var languages := []
	for repo in account.repos:
		if not repo.owner.login in owners:
			owners.append(repo.owner.login)
		var lang = repo.language if repo.language else "Unknown"
		if not lang in languages:
			languages.append(lang)
	owners.sort()
	languages.sort()
	owners.push_front("All")
	languages.push_front("All")
	var choices := {
		"Owner": owners,
		"Language": languages,
		"Visibility": ["All", "Public", "Private"],
		"Sort": ["Updated", "Name", "Stars", "Size"]
	}
	for key in choices:
		var group := column(bar, 2)
		label(group, key, 13, muted)
		pick(group, choices[key], filters[key], _filter_changed.bind(key))
	button(
		bar,
		"Descending" if descending else "Ascending",
		func():
			descending = not descending
			draw_page()
	)
	var actions := HFlowContainer.new()
	content.add_child(actions)
	button(actions, "Select filtered", _select_many.bind(false))
	button(actions, "Pushed in 30 days", _select_many.bind(true))
	button(
		actions,
		"Clear selection",
		func():
			selected.clear()
			_populate_repos()
	)
	button(actions, "Configure backup", go.bind("Backup"), true)
	selection_label = label(content, "", 15, muted)
	repo_list = _table(["Repository", "Visibility", "Language", "Stars", "Size"], 400)
	repo_list.set_column_expand_ratio(0, 3)
	repo_list.item_edited.connect(_edited)
	_populate_repos()


func _table(titles: Array, height: int) -> Tree:
	var tree := Tree.new()
	tree.columns = titles.size()
	tree.hide_root = true
	tree.column_titles_visible = true
	tree.custom_minimum_size.y = height
	for i in titles.size():
		tree.set_column_title(i, titles[i])
	content.add_child(tree)
	return tree


func _filter_changed(value, key):
	filters[key] = value
	_populate_repos()


func _select_many(recent: bool):
	for repo in _filtered():
		if (
			recent
			and (
				not repo.pushed_at
				or (
					Time.get_unix_time_from_datetime_string(repo.pushed_at)
					< Time.get_unix_time_from_system() - 2592000
				)
			)
		):
			continue
		selected[str(repo.id)] = true
	_populate_repos()


func _populate_repos():
	repo_list.clear()
	var root = repo_list.create_item()
	for repo in _filtered():
		var item := repo_list.create_item(root)
		var id := str(repo.id)
		item.set_cell_mode(0, TreeItem.CELL_MODE_CHECK)
		item.set_editable(0, true)
		item.set_checked(0, selected.has(id))
		item.set_metadata(0, id)
		item.set_tooltip_text(0, str(repo.description))
		var values = [
			repo.full_name,
			"Private" if repo.private else "Public",
			repo.language if repo.language else "—",
			str(int(repo.stargazers_count)),
			String.humanize_size(int(repo.size) * 1024)
		]
		for i in values.size():
			item.set_text(i, values[i])
	selection_label.text = "%d selected" % selected.size()


func _edited():
	var item := repo_list.get_edited()
	var id = item.get_metadata(0)
	if item.is_checked(0):
		selected[id] = true
	else:
		selected.erase(id)
	selection_label.text = "%d selected" % selected.size()


func _backup_page():
	label(content, "Backup", 36)
	var tabs := row(content)
	for name_ in ["Options", "Progress", "History"]:
		button(tabs, name_, _set_tab.bind(name_), tab == name_)
	match tab:
		"History":
			_history(50)
		"Progress":
			progress_label = label(content, "", 17, muted)
			progress_bar = ProgressBar.new()
			content.add_child(progress_bar)
			progress_list = _table(["Repository", "Status", "Details"], 300)
			_update_progress()
			if backup.running:
				button(content, "Stop operation", func(): backup.cancelled = true)
		"Options":
			var group := card(content)
			label(group, "Destination", 20)
			var dest_row := row(group)
			var dest := field(dest_row, options.destination, "Select a backup folder")
			dest.text_changed.connect(func(value): options.destination = value)
			button(dest_row, "Browse", _folder.bind(false))
			var format_row := row(group)
			label(format_row, "Clone type")
			pick(
				format_row,
				["full", "mirror"],
				options.cloneType,
				func(val): options.cloneType = val
			)
			label(format_row, "Group by")
			pick(format_row, ["owner", "none"], options.splitBy, func(val): options.splitBy = val)
			_number(
				group,
				"Concurrent repositories",
				1,
				8,
				int(options.maxConcurrent),
				func(val): options.maxConcurrent = int(val)
			)
			check(
				group, "Include forks", options.includeForks, func(val): options.includeForks = val
			)
			check(
				group,
				"Include archived repositories",
				options.includeArchived,
				func(val): options.includeArchived = val
			)
			var archive := card(content)
			check(
				archive,
				"Create ZIP archive",
				options.createZip,
				func(val):
					options.createZip = val
					draw_page()
			)
			if options.createZip:
				_number(
					archive,
					"Compression level",
					0,
					9,
					int(options.zipCompression),
					func(val): options.zipCompression = int(val)
				)
				label(archive, "Exclude names or patterns (comma separated)", 14, muted)
				var excludes := field(
					archive, ", ".join(options.excludePatterns), "node_modules, *.tmp"
				)
				excludes.text_changed.connect(
					func(value):
						options.excludePatterns = []
						for part in value.split(",", false):
							if not part.strip_edges().is_empty():
								options.excludePatterns.append(part.strip_edges())
				)
			button(content, "Start backup · %d selected" % selected.size(), _start, true).disabled = (
				backup.running or selected.is_empty()
			)


func _number(parent: Node, title: String, low: int, high: int, value: int, fn: Callable):
	var line := row(parent)
	label(line, title)
	var spin := SpinBox.new()
	spin.min_value = low
	spin.max_value = high
	spin.value = value
	spin.value_changed.connect(fn)
	line.add_child(spin)


func _folder(for_default: bool):
	var dialog := FileDialog.new()
	dialog.file_mode = FileDialog.FILE_MODE_OPEN_DIR
	dialog.access = FileDialog.ACCESS_FILESYSTEM
	dialog.use_native_dialog = true
	dialog.title = "Select backup destination"
	add_child(dialog)
	dialog.dir_selected.connect(
		func(path):
			if for_default:
				store.settings.defaultBackupLocation = path
				persist()
			else:
				options.destination = path
			draw_page()
			dialog.queue_free()
	)
	dialog.canceled.connect(dialog.queue_free)
	dialog.popup_centered_ratio(0.7)


func _set_tab(value: String):
	tab = value
	go("Backup")


func _start():
	if backup.running:
		return
	var batch := []
	for repo in account.repos:
		if not selected.has(str(repo.id)):
			continue
		if (
			(repo.fork and not options.includeForks)
			or (repo.archived and not options.includeArchived)
		):
			continue
		batch.append(repo)
	if options.destination.is_empty() or not options.destination.is_absolute_path():
		feedback = "Choose an absolute destination folder."
	elif batch.is_empty():
		feedback = "No selected repositories match your backup options."
	else:
		tab = "Progress"
		feedback = ""
		draw_page()
		backup.start(batch, options, hub.token)
		return
	draw_page()


func _update_progress():
	if page != "Backup" or tab != "Progress" or not is_instance_valid(progress_list):
		return
	progress_list.clear()
	var root := progress_list.create_item()
	var count := 0
	for result in backup.rows:
		var item := progress_list.create_item(root)
		item.set_text(0, result.repo.full_name)
		item.set_text(1, result.status)
		item.set_text(2, result.error)
		if result.status in ["Complete", "Failed", "Cancelled"]:
			count += 1
	var total: int = backup.rows.size()
	progress_bar.value = 100.0 * count / maxi(1, total)
	if total == 0:
		progress_label.text = "No backup in progress."
		return
	var elapsed := int((Time.get_ticks_msec() - backup.started) / 1000.0)
	var detail: String = backup.message
	if backup.running:
		detail = "Stopping after active clones finish…" if backup.cancelled else detail
		detail += "   %ds elapsed" % elapsed
		if count > 0 and count < total:
			detail += " · ~%ds remaining" % int(elapsed * float(total - count) / count)
	progress_label.text = "%d / %d    %s" % [count, total, detail]


func _close_requested():
	if not backup.running:
		get_tree().quit()
	elif not quit_after_backup:
		_confirm(
			"Stop backup and quit?",
			"Active clones will finish before the application closes.",
			func():
				quit_after_backup = true
				backup.cancelled = true
				_set_tab("Progress")
		)


func _finished(entry: Dictionary):
	store.data.backup_history.push_front(entry)
	store.data.backup_history.resize(mini(50, store.data.backup_history.size()))
	persist()
	if quit_after_backup:
		get_tree().quit()
	else:
		draw_page()


func _history(limit: int):
	if store.data.backup_history.is_empty():
		label(content, "No backups recorded yet.", 16, muted)
		return
	for entry in store.data.backup_history.slice(0, limit):
		var item := card(content)
		var line := row(item)
		fill(label(line, "%s  ·  %s" % [str(entry.status).capitalize(), entry.date], 18))
		button(line, "Open folder", OS.shell_show_in_file_manager.bind(entry.destination, true))
		label(
			item,
			(
				"%d repositories   ·   %.1f s   ·   Run %s"
				% [entry.repoCount, float(entry.duration) / 1000, entry.id]
			),
			15,
			muted
		)
		var path := label(item, entry.destination, 14, muted)
		path.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART


func _settings():
	label(content, "Settings", 36)
	var group := card(content)
	label(group, "Appearance", 19)
	pick(
		group,
		["system", "light", "dark"],
		store.settings.theme,
		func(val):
			store.settings.theme = val
			persist()
			draw_page()
	)
	check(
		group,
		"Notifications",
		store.settings.notifications,
		func(val):
			store.settings.notifications = val
			persist()
	)
	var location := card(content)
	label(location, "Default backup folder", 19)
	label(
		location,
		(
			store.settings.defaultBackupLocation
			if store.settings.defaultBackupLocation
			else "Choose a default folder"
		),
		16,
		muted
	)
	button(location, "Browse", _folder.bind(true))
	var profile := card(content)
	label(profile, "Signed in as @" + str(account.user.login), 18)
	button(
		profile,
		"Clear backup history",
		func():
			_confirm("Clear backup history?", "Local backups will stay on disk.", _clear_history)
	)
	button(profile, "Disconnect account", func(): _confirm("Disconnect account?", "This removes the saved token from this computer.", _logout)).disabled = (
		backup.running
	)


func _confirm(title: String, body: String, action: Callable):
	var dialog := ConfirmationDialog.new()
	dialog.title = title
	dialog.dialog_text = body
	add_child(dialog)
	dialog.confirmed.connect(action)
	dialog.confirmed.connect(dialog.queue_free)
	dialog.canceled.connect(dialog.queue_free)
	dialog.popup_centered()


func _clear_history():
	store.data.backup_history.clear()
	persist()
	draw_page()


func _logout():
	store.data.token = null
	if store.save() != OK:
		feedback = "Could not remove the saved token."
		draw_page()
		return
	auth_generation += 1
	busy = false
	hub.token = ""
	account.clear()
	selected.clear()
	backup.rows.clear()
	device = {}
	query = ""
	filters = {"Owner": "All", "Visibility": "All", "Language": "All", "Sort": "Updated"}
	go("Connect")


func _about():
	label(content, "About", 36)
	var section := card(content)
	label(section, "Gosh GitHub Backup Manager", 26, accent)
	label(section, "Version 2.1.0   ·   Godot 4.7.2   ·   AGPL-3.0", 16, muted)
	button(section, "View on GitHub", OS.shell_open.bind(PROJECT))
	button(section, "Report an issue", OS.shell_open.bind(PROJECT + "/issues"))
