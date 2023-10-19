import { CodegenConfig } from '@graphql-codegen/cli';

 const config: CodegenConfig = {
  schema: process.env.SCHEMA_PATH,
  documents: ['src/**/*.{ts,tsx}'],
  generates: {
    './src/__generated__/': {
      preset: 'client',
      plugins: [],
      presetConfig: {
        gqlTagName: 'gql',
      }
    }
  },
  ignoreNoDocuments: true,
};

export default config
