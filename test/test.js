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


test('get_post', (t) => {
  t.plan(2)

  let post_hash = "abcd1234"
  const params = JSON.stringify({post_hash})

  const content = "Holo world"
  const in_reply_to = ""
  const create_post_result = app.call("blog", "main", "create_post", JSON.stringify({content, in_reply_to}))

  post_hash = create_post_result.hash
  const result = app.call("blog", "main", "get_post", JSON.stringify({post_hash}))

  t.equal(result.content, content)
  t.equal(result.in_reply_to, in_reply_to)
})
