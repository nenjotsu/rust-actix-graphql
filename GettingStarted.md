# Create User
```
POST http://localhost:3000/user/register

{
	"name": "nenjo",
	"email": "email@gmail.com",
	"password": "password123"
}

```

# Login 
```
POST  http://localhost:8000/user/login
{
	"email": "email@gmail.com",
	"password": "password123"
}
```

# Logout
```
GET http://localhost:8000/user/logout
```

# GraphQL
```
POST http://localhost:8000/graphql

query usersQuery {
	users(limit: 10, offset: 0) {
		name
		createdAt
		email
		role
		userId
		userUuid
	}
}

query tokenQuery {
	generateToken {
		bearer
	}
}

query decodeToken {
	decodeToken {
		email
		exp
		iat
		iss
		sub
	}
}

query postsQuery {
	posts(limit: 10, offset: 0, keyword: "") {
		body
		id
		postUuid
		published
		title
	}
}

mutation createPost {
	createPost(data: {
		body: "pink body",
		title: "pink post"
	}) {
		id
		postUuid
		title
		body
	}
}


mutation updatePost {
	updatePost(id: 3, data: {
		body: "mejo green body dark",
		title: "mejo green post dark",
	}) {
		postUuid
		title
		body
	}
}

mutation deletePost {
	deletePost(id: 1) {
		id
		postUuid
		title
	}
}

```