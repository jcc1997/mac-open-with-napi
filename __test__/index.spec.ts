import test from 'ava'

import { urlsForFile } from '../index'

test('sync function from native code', (t) => {
  const result = urlsForFile('/Users/gd/Desktop/github/mac-open-with-napi/__test__/index.spec.ts')
  console.warn(result)
  t.assert('true', 'true')
})
