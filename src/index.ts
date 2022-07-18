import 'reflect-metadata'
import 'dotenv/config'

import express from 'express'
import cors from 'cors'
import { ApolloServer } from 'apollo-server-express'
import { buildSchema } from 'type-graphql'

import { ThreadResolver } from './gql/resolvers/thread_resolver'
;(async () => {
	const app = express()
	app.use(
		cors({
			origin: '*',
		})
	)

	const apollo_server = new ApolloServer({
		schema: await buildSchema({
			resolvers: [ThreadResolver],
			validate: false,
		}),
		context: ({ req, res }) => ({ req, res }),
	})
	await apollo_server.start()
	apollo_server.applyMiddleware({
		app,
		cors: false,
	})

	app.listen(parseInt(process.env.PORT!), () => {
		console.log(`running at ${process.env.PORT}`)
	})
})().catch(console.error)
