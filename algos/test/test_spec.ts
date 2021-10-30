import assert from 'assert';
import helloWorld from './test';

describe('Test', () => {
  it('should return "hello world"', () => {
    assert(helloWorld() === 'hello world');
  });
});
