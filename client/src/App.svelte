<script lang="ts">
  import Home from "./Home.svelte";
  import { Router, Route } from "svelte-routing";
  import Test from "./Test.svelte";

  export let url = "";
</script>

<Router {url}>
  <Route path="/"><Home /></Route>
  <Route path="/test"><Test /></Route>
</Router>
