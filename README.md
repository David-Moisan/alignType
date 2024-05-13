# Align Type: Keeping Your GraphQL and TypeScript Types in Sync with a Rust Twist

Are you tired of the repetitive task of aligning your GraphQL types with your TypeScript types? Enter Align Type - a utility crafted with care in the depths of Rust to ease your pain and bring harmony to your codebase.

## Usage

Currently at version 0.0.1, Align Type is just getting warmed up, but fear not, there's plenty of magic to come! While I iron out the kinks, here's how you can get started:


```bash
./align_type [path_to_graphql_schema] [path_to_typescript_type_file]
```

## What Align Type Does

Align Type takes your GraphQL type definitions, such as:

```graphql
type Test {
    id: ID!
    name: String!
    firstname: String
    roles: [Role]!
    age: Int
    is_vip: Boolean 
}
```

And converts them into TypeScript type declarations like:

```ts
export type Test = {
    id: number,
    name: string,
    firstname: string | null,
    roles: Role[],
    age: number | null,
    is_vip: boolean,
};
```

## The Future Looks Bright

- [ ] SQL mapping for those SQLite enthusiasts out there who brave the wild without an ORM and need to juggle types for SQLite, GraphQL, and TypeScript (yes, that's me too!)

- [ ] Crafting Linux packages and Windows .exe files to ensure Align Type can strut its stuff wherever code is written.

We're open to your suggestions for improvements and eagerly await any bug reports you may have. Let's make Align Type the hero your code deserves! 🦾
