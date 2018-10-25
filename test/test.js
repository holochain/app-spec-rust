const test = require('tape');

const app = Container.loadAndInstantiate("dist/app-spec-rust.hcpkg")
app.start()

test('get entry address', (t) => {
  t.plan(1)

  const result = app.call("blog", "main", "hash_post", "Holo world")

  t.equal(result, JSON.stringify({"address":"QmZi7c1G2qAN6Y5wxHDB9fLhSaSVBJe28ZVkiPraLEcvou"}))
})

test('create_post', (t) => {
  t.plan(1)

  const content = "Holo world"
  const in_reply_to = ""
  const params = JSON.stringify({content, in_reply_to})
  const result = app.call("blog", "main", "create_post", params)

  t.equal(result, JSON.stringify({"hash":"QmdJHaznj5rAtMV5nXLK87tdCBoc2NJRtQW4r3w7LZ6HSg"}))
})

test('post max content size 280 characters', (t) => {
  t.plan(1)

  const content = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."
  const in_reply_to = ""
  const params = JSON.stringify({content, in_reply_to})
  const result = app.call("blog", "main", "create_post", params)

  t.equal(result, JSON.stringify({"error": {"ValidationFailed": "\"Content too long\""}}))
})

test('posts_by_agent', (t) => {
  t.plan(1)

  const agent = "Bob"
  const params = JSON.stringify({agent})

  const result = app.call("blog", "main", "posts_by_agent", params)

  t.equal(result, JSON.stringify({"post_hashes":[]}))
})

test('my_posts', (t) => {
  t.plan(1)

  app.call("blog", "main", "create_post",
    JSON.stringify({"content": "Holo world", "in_reply_to": ""})
  )

  app.call("blog", "main", "create_post",
    JSON.stringify({"content": "Another post", "in_reply_to": ""})
  )

  const result = app.call("blog", "main", "my_posts", JSON.stringify({}))
  const ordering1 = result == JSON.stringify({"post_hashes":["Qme9vatSfYs7MpejUUrheYYUA1B2TYdVBDycuoimtHudMP","QmdJHaznj5rAtMV5nXLK87tdCBoc2NJRtQW4r3w7LZ6HSg"]})
  const ordering2 = result == JSON.stringify({"post_hashes":["QmdJHaznj5rAtMV5nXLK87tdCBoc2NJRtQW4r3w7LZ6HSg","Qme9vatSfYs7MpejUUrheYYUA1B2TYdVBDycuoimtHudMP"]})
  t.ok(ordering1 || ordering2, "Did not get post hashes [\"QmdJHaznj5rAtMV5nXLK87tdCBoc2NJRtQW4r3w7LZ6HSg\",\"Qme9vatSfYs7MpejUUrheYYUA1B2TYdVBDycuoimtHudMP\"] in any ordering")
})


test('create/get_post rountrip', (t) => {
  t.plan(3)

  const content = "Holo world"
  const in_reply_to = ""
  const params = JSON.stringify({content, in_reply_to})
  const create_post_result = app.call("blog", "main", "create_post", params)

  t.equal(
    create_post_result,
    JSON.stringify({"hash":"QmdJHaznj5rAtMV5nXLK87tdCBoc2NJRtQW4r3w7LZ6HSg"})
  )
  post_hash = JSON.parse(create_post_result)["hash"]
  t.equal(
    post_hash,
    "QmdJHaznj5rAtMV5nXLK87tdCBoc2NJRtQW4r3w7LZ6HSg"
  )

  const params_get = JSON.stringify({post_hash})
  const result = app.call("blog", "main", "get_post", params_get)

  const entry = JSON.parse(result)
  t.equal(entry.content, content)
})


test('get_post with non-existant hash returns empty object', (t) => {
  t.plan(1)

  const post_hash = "RANDOM"
  const params_get = JSON.stringify({post_hash})
  const result = app.call("blog", "main", "get_post", params_get)

  const entry = JSON.parse(result)
  t.same(entry, {})
})
