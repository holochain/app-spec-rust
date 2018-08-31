var test = require('tape');

let app = Container.loadAndInstantiate("dist/bundle.json")
app.start()

test('create_post', function(t) {
  t.plan(1)

  let content = "Holo world"
  let in_reply_to = ""
  let params = JSON.stringify({content, in_reply_to})
  let result = app.call("blog", "main", "create_post", params)

  t.equal(result, JSON.stringify({"error": "commit failed"}))
})
