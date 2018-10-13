const test = require('tape');

const app = Container.loadAndInstantiate("dist/app-spec-rust.hcpkg")
app.start()

test('create_post', (t) => {
  t.plan(1)

  const content = "Holo world"
  const in_reply_to = ""
  const params = JSON.stringify({content, in_reply_to})
  const result = app.call("blog", "main", "create_post", params)

  t.equal(result, JSON.stringify({"hash":"Qma2xMsbBGp2baimoKhiZnMCzdcepvPzDXgktVrz3wQH8E"}))
})

test('posts_by_agent', (t) => {
  t.plan(1)

  const agent = "Bob"
  const params = JSON.stringify({agent})

  const result = app.call("blog", "main", "posts_by_agent", params)

  t.equal(result, JSON.stringify({"post_hashes": []}))
})


test('create/get_post rountrip', (t) => {
  t.plan(3)

  const content = "Holo world"
  const in_reply_to = ""
  const params = JSON.stringify({content, in_reply_to})
  const create_post_result = app.call("blog", "main", "create_post", params)

  t.equal(
    create_post_result,
    JSON.stringify({"hash":"Qma2xMsbBGp2baimoKhiZnMCzdcepvPzDXgktVrz3wQH8E"})
  )
  post_hash = JSON.parse(create_post_result)["hash"]
  t.equal(
    post_hash,
    "Qma2xMsbBGp2baimoKhiZnMCzdcepvPzDXgktVrz3wQH8E"
  )

  const params_get = JSON.stringify({post_hash})
  const result = app.call("blog", "main", "get_post", params_get)

  const entry = JSON.parse(result)
  t.equal(entry.content, content)
})



})
