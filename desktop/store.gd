extends RefCounted

const PATH = "user://gosh-github-backup-manager.json"
const OPTIONS = {
	"cloneType": "full",
	"includeForks": true,
	"includeArchived": false,
	"splitBy": "owner",
	"createZip": false,
	"zipCompression": 6,
	"excludePatterns": ["node_modules", ".DS_Store", "Thumbs.db"],
	"maxConcurrent": 3
}
var data = {"token": null, "settings": {}, "backup_history": []}
var settings: Dictionary
var problem := ""


func _init():
	if FileAccess.file_exists(PATH):
		var loaded = JSON.parse_string(FileAccess.get_file_as_string(PATH))
		if loaded is Dictionary and loaded.get("backup_history", []) is Array:
			data.merge(loaded, true)
		else:
			problem = "Your saved profile could not be read. It has been preserved."
	settings = data.settings if data.settings is Dictionary else {}
	settings.merge(
		{
			"theme": "system",
			"defaultBackupLocation": "",
			"notifications": true,
			"autoBackup": {"enabled": false, "schedule": "weekly", "time": "03:00", "lastRun": null}
		}
	)
	var defaults = settings.get("defaultBackupOptions")
	if not defaults is Dictionary:
		defaults = {}
	for key in OPTIONS:
		if defaults.get(key) == null:
			defaults[key] = OPTIONS[key]
	settings.defaultBackupOptions = defaults
	data.settings = settings


func save() -> Error:
	if not problem.is_empty():
		return ERR_FILE_CORRUPT
	var temp = PATH + "." + Crypto.new().generate_random_bytes(12).hex_encode()
	var file := FileAccess.open(temp, FileAccess.WRITE)
	if not file:
		return FileAccess.get_open_error()
	var error := OK
	if OS.get_name() != "Windows":
		error = FileAccess.set_unix_permissions(
			temp, FileAccess.UNIX_READ_OWNER | FileAccess.UNIX_WRITE_OWNER
		)
	if error == OK:
		file.store_string(JSON.stringify(data))
		file.flush()
		error = file.get_error()
	file.close()
	if error == OK:
		error = DirAccess.rename_absolute(
			ProjectSettings.globalize_path(temp), ProjectSettings.globalize_path(PATH)
		)
	if error != OK:
		DirAccess.remove_absolute(ProjectSettings.globalize_path(temp))
	return error
