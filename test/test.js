var test = require('tape');

let app = Container.loadAndInstantiate("dist/bundle.json")
app.start()

test('create_post', function(t) {
  //t.plan(2)

  let content = "Holo world"
  let in_reply_to = ""
  let params = JSON.stringify({content, in_reply_to})
  console.log("params: " + params)
  let result = app.call("blog", "main", "create_post", params)

  t.equal(result, JSON.stringify({"error": "commit failed"}))

  console.log("result: " + result)
})
