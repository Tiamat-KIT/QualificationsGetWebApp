import { defineSchema,defineTable } from "convex/server";
import {v} from "convex/values"

export default defineSchema({
    exam: defineTable({
        title: v.string(),
        description: v.string(),
        quizes_ids: v.array(v.id("quiz"))
    }),
    quiz: defineTable({
        exam_id: v.id("exam"),
        title: v.string(),
        description: v.string(),
        answer: v.union(v.string(),v.number()),
        answer_description: v.string()
    })
})