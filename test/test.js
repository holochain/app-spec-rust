const test = require('tape');

const app = Container.loadAndInstantiate("dist/app-spec-rust.hcpkg")
app.start()

test('create_post', (t) => {
  t.plan(1)

  const content = "Holo world"
  const in_reply_to = ""
  const params = JSON.stringify({content, in_reply_to})
  const result = app.call("blog", "main", "create_post", params)

  t.equal(result, JSON.stringify({"error": "commit failed"}))
})

test('posts_by_agent', (t) => {
  t.plan(1)

  const agent = "Bob"
  const params = JSON.stringify({agent})

  const result = app.call("blog", "main", "posts_by_agent", params)

  t.equal(result, JSON.stringify({"post_hashes": []}))
})


test('posts_by_agent', (t) => {
  t.plan(1)

  const post_hash = "abcd1234"
  const params = JSON.stringify({post_hash})

  const result = app.call("blog", "main", "get_post", params)

  t.equal(result, JSON.stringify({"error": "Function not implemented"}))
})
