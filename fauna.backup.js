import { Client, fql } from "fauna";
import {createClient} from "@supabase/supabase-js";

const fauna = new Client({ secret: "fnAFOs9NL0AAzIHsHDcjUBAiaYJ4Tzxek9uXQoPc" });
const supabase = createClient("https://fqhsguucsbzmsbtefhjt.supabase.co", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImZxaHNndXVjc2J6bXNidGVmaGp0Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2OTgxNTY5MDgsImV4cCI6MjAxMzczMjkwOH0.teUl645OFTVqBZUajst18kAySURtLktM4OgId-StvkY")

const ids = await fauna
	.query(
		fql`
			signatures.all().paginate(1000) { data { id } }`
	).then(res => res.data.data).then(list => list.map(obj => obj.id))

console.log(ids);

ids.forEach(async (id) => {

	const signature = await fauna.query(fql`signatures.where(.id == ${id}).first()
			`,
		{ format: "simple" }).then(res => res.data)

	// console.log(signature);

	const res = await supabase.from("signatures").insert({
		name: signature.name,
		signature: signature.signature,
		ts_created: new Date(signature.ts_created / 1000),
		ts_modified: new Date(signature.ts_moderated / 1000),
		approved: signature.status == "declined" ? false : signature.status == "approved" ? true : null
	})

	console.log(res);
})
