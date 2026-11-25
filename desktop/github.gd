extends Node

const CLIENT = "Ov23liGgLL9AjIdQnRFt"
var api_root := "https://api.github.com"
var device_root := "https://github.com"
var token := ""


func request_json(url: String, form := {}) -> Dictionary:
	var http := HTTPRequest.new()
	add_child(http)
	http.timeout = 30
	http.max_redirects = 0
	var proxy := OS.get_environment("HTTPS_PROXY")
	if proxy.is_empty():
		proxy = OS.get_environment("https_proxy")
	if proxy.begins_with("http://"):
		var addr := proxy.trim_prefix("http://").trim_suffix("/")
		if not addr.contains("@"):
			http.set_https_proxy(
				addr.get_slice(":", 0), int(addr.get_slice(":", 1)) if addr.contains(":") else 80
			)
	var headers = ["User-Agent: Gosh-GitHub-Backup-Manager", "Accept: application/json"]
	if url.begins_with(api_root + "/"):
		headers.append("Authorization: Bearer " + token)
		headers.append("X-GitHub-Api-Version: 2022-11-28")
	if not form.is_empty():
		headers.append("Content-Type: application/x-www-form-urlencoded")
	var error = http.request(
		url,
		headers,
		HTTPClient.METHOD_GET if form.is_empty() else HTTPClient.METHOD_POST,
		HTTPClient.new().query_string_from_dict(form)
	)
	if error != OK:
		http.queue_free()
		return {"error": error_string(error)}
	var res = await http.request_completed
	http.queue_free()
	if res[0] != HTTPRequest.RESULT_SUCCESS:
		return {"error": "Connection to GitHub failed (%s)." % res[0]}
	var body = JSON.parse_string(res[3].get_string_from_utf8())
	if res[1] < 200 or res[1] >= 300:
		return {
			"error":
			(
				"GitHub returned HTTP %d. %s"
				% [res[1], body.get("message", "") if body is Dictionary else ""]
			)
		}
	if body == null:
		return {"error": "GitHub returned an unreadable response."}
	return {"value": body}


func account() -> Dictionary:
	var session_token := token
	var user = await request_json(api_root + "/user")
	if user.has("error"):
		return user
	if not user.value is Dictionary or not user.value.has("login"):
		return {"error": "GitHub returned an invalid account."}
	var result = {
		"user": user.value, "repos": [], "orgs": [], "events": [], "warnings": [], "unavailable": []
	}
	var repos := []
	var page := 1
	while session_token == token:
		var batch = await request_json(
			(
				api_root
				+ (
					"/user/repos?visibility=all&affiliation=owner,collaborator,organization_member&sort=updated&per_page=100&page=%d"
					% page
				)
			)
		)
		if batch.has("error") or not batch.value is Array:
			result.unavailable.append("repos")
			result.warnings.append("Repositories: " + batch.get("error", "Unexpected response."))
			break
		repos.append_array(batch.value)
		if batch.value.size() < 100:
			result.repos = repos
			break
		page += 1
	for pair in [
		["orgs", "/user/orgs?per_page=100"],
		["events", "/users/" + str(user.value.login).uri_encode() + "/events?per_page=20"]
	]:
		if session_token != token:
			return {"error": "Account changed."}
		var response = await request_json(api_root + pair[1])
		if response.has("error") or not response.value is Array:
			result.unavailable.append(pair[0])
			result.warnings.append(pair[0] + ": " + response.get("error", "Unexpected response."))
		else:
			result[pair[0]] = response.value
	return result
