const debug = require("debug")("canduma:post.test.js");
const request = require("supertest");
const HttpStatus = require("http-status-codes");
const apiPort = process.env.PORT || 8000;
const url = `http://localhost:${apiPort}`;

jest.setTimeout(1000);

const postBody = "pink body";
const postTitle = "pink title";

const email = "nenjotech@gmail.com";
const password = "Qweqwe29";

async function cleanupDb() {
  return global.knex.raw("DELETE FROM posts WHERE title in (?)", [postTitle]);
}
describe("post/", () => {
  const agent = request.agent(url);
  beforeAll(async () => {
    return cleanupDb();
  });
  afterAll(async () => {
    return cleanupDb();
  });

  it("/graphql:M createPost - OK", async done => {
    const { body } = await agent
      .post("/graphql")
      .send({
        query: `mutation  {
            createPost( data: { title: "${postTitle}", body: "${postBody}" }) {
                postUuid
                title
                body
            }
        }`
      })
      .expect(HttpStatus.OK);
    debug("/graphql posts=%o", body);
    const {
      data: { createPost }
    } = body;
    expect(createPost).toContainAllKeys(["postUuid", "title", "body"]);
    // expect(createPost.id).toBeNonEmptyString();
    expect(createPost.postUuid).toBeNonEmptyString();
    expect(createPost.title).toBe(postTitle);
    expect(createPost.body).toBe(postBody);
    // expect(createPost.published).toBe(false);
    done();
  });

  it("/graphql:Q posts - OK", async done => {
    const response1 = await agent
      .post("/graphql")
      .send({
        query: `query postsQuery {
          posts(limit: 10, offset: 0, keyword: "") {
            body
            id
            postUuid
            published
            title
          }
        }`
      })
      .expect(HttpStatus.OK);
    debug("/graphql posts=%o", response1.body.data.posts);
    expect(response1.body.data.posts).toBeNonEmptyArray();

    const { posts } = response1.body.data;

    expect(posts[0]).toContainAllKeys([
      "id",
      "postUuid",
      "title",
      "body",
      "published"
    ]);
    expect(posts[0].id).toBeDefined();
    expect(posts[0].postUuid).toBeDefined();
    expect(posts[0].title).toBeDefined();
    expect(posts[0].body).toBeDefined();
    expect(posts[0].published).toBe(false);
    done();
  });

  it("/graphql:Q posts with keyword - OK", async done => {
    const response1 = await agent
      .post("/graphql")
      .send({
        query: `query postsQuery {
          posts(limit: 10, offset: 0, keyword: "pink") {
            body
            id
            postUuid
            published
            title
          }
        }`
      })
      .expect(HttpStatus.OK);
    debug("/graphql posts=%o", response1.body.data.posts);
    expect(response1.body.data.posts).toBeNonEmptyArray();

    const { posts } = response1.body.data;

    expect(posts[0].title).toContain("pink");
    done();
  });

  it("/graphql:M posts update - OK", async done => {
    const { body } = await agent
      .post("/graphql")
      .send({
        query: `mutation updatePost {
          updatePost(id: 3, data: {
            body: "mejo green body dark",
            title: "mejo green post dark",
          }) {
            postUuid
            title
            body
          }
        }`
      })
      .expect(HttpStatus.OK);
    debug("/graphql posts=%o", body);
    const {
      data: { updatePost }
    } = body;
    expect(updatePost).toContainAllKeys(["postUuid", "title", "body"]);
    expect(updatePost.title).toContain("green");
    expect(updatePost.body).toContain("green");
    done();
  });


  it("/graphql:M posts create & delete - OK", async done => {
    const { body: createBody } = await agent
      .post("/graphql")
      .send({
        query: `mutation  {
            createPost( data: { title: "${postTitle}", body: "${postBody}" }) {
                id
                postUuid
                title
                body
            }
        }`
      })
      .expect(HttpStatus.OK);
    debug("/graphql posts=%o", createBody);
    const {
      data: { createPost }
    } = createBody;
    expect(createPost).toContainAllKeys(["id", "postUuid", "title", "body"]);
    expect(createPost.id).toBeDefined();

    const { body } = await agent
      .post("/graphql")
      .send({
        query: `mutation deletePost {
          deletePost(id: ${createPost.id}) {
            id
            postUuid
            title
            body
          }
        }`
      })
      .expect(HttpStatus.OK);
    debug("/graphql posts=%o", body);
    const {
      data: { deletePost }
    } = body;
    expect(deletePost).toContainAllKeys(["id", "postUuid", "title", "body"]);
    expect(deletePost.id).toBe(createPost.id);
    done();
  });
});
