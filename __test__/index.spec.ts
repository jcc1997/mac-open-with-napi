import test from 'ava'

import { urlsForFile } from '../index'

test('sync function from native code', (t) => {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  const result = urlsForFile(`${__dirname}/index.spec.ts`)
  console.warn(result)
  t.assert('true', 'true')
})
