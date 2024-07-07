import { mutation, query } from "./_generated/server";
import {v} from "convex/values"

export const addQuiz = mutation({
    args: {
        title: v.string(),
        description: v.string(),
        answer: v.union(v.string(),v.number()),
        answer_description: v.string(),
        exam_id: v.id("exam")
    },
    handler: async (ctx,args) => {
        await ctx.db.insert("quiz",{
            title: args.title,
            description: args.description,
            answer: args.answer,
            answer_description: args.answer_description,
            exam_id: args.exam_id
        })
    }
})

export const getQuizes = query({
    args: {
        exam_id: v.id("exam")
    },
    handler: async (ctx,args) => {
        return ctx.db.get(args.exam_id)
    }
})

export const getQuize = query({
    args: {
        quiz_id: v.id("quiz")
    },
    handler: async(ctx, args) =>{
        const result = await ctx.db.get(args.quiz_id)
        if(result === null){
            throw new Error()
        }
        return result
    },
})