/// <reference types="vite/client" />

interface BigInt {
  toJSON: () => string;
}

BigInt.prototype.toJSON = function () {
  return this.toString();
};
