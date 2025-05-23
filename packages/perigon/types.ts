import type { FromSchema } from 'json-schema-to-ts';
import * as schemas from './schemas';

export type AllNewsMetadataParam = FromSchema<typeof schemas.AllNews.metadata>;
export type AllNewsResponse200 = FromSchema<typeof schemas.AllNews.response['200']>;
export type AllNewsResponse400 = FromSchema<typeof schemas.AllNews.response['400']>;
export type AllNewsResponse401 = FromSchema<typeof schemas.AllNews.response['401']>;
export type AllNewsResponse403 = FromSchema<typeof schemas.AllNews.response['403']>;
export type AllNewsResponse404 = FromSchema<typeof schemas.AllNews.response['404']>;
export type AllNewsResponse500 = FromSchema<typeof schemas.AllNews.response['500']>;
export type Stories1MetadataParam = FromSchema<typeof schemas.Stories1.metadata>;
export type Stories1Response200 = FromSchema<typeof schemas.Stories1.response['200']>;
export type Stories1Response400 = FromSchema<typeof schemas.Stories1.response['400']>;
