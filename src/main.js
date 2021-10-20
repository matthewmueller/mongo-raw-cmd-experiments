import { MongoClient } from "mongodb"
import getStdin from "get-stdin"

const url =
  "mongodb://prisma:prisma@127.0.0.1:27017/testdb?authSource=admin&retryWrites=true"

const client = new MongoClient(url, {
  useUnifiedTopology: true,
  useNewUrlParser: true,
})
async function main() {
  // Connect using MongoClient
  const doc = JSON.parse(process.argv[2] || (await getStdin()))

  // Connect to the MongoDB client
  await client.connect()

  // Select the db, when blank, it uses the connection string URL
  const db = client.db()

  // This is where the MongoDB JS driver seems slightly higher-level
  if ("find" in doc) {
    const collection = db.collection(doc["find"])
    const result = await collection.find(doc["filter"]).toArray()
    console.log(result)
    return
  }
  if ("insert" in doc) {
    const collection = db.collection(doc["insert"])
    const result = await collection.insertMany(doc["documents"])
    console.log(result)
    return
  }
}

main()
  .catch(console.error)
  .finally(() => client.close())
