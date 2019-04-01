import("../crate/pkg").then(module => {
  module.run();
});

import("./app").then(() => {
  console.log('Loaded App Module!')
})
