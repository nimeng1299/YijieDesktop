<script>
      let {tabId, room} = $props();
</script>
<div class="game-container">
  <div class="game-left">左侧</div>
  <div class="game-middle">中间</div>
  <div class="game-right">右侧</div>
</div>

<style>
  .game-container {
    display: flex;
  }
  .game-left {
    width: 25%;
  }
  .game-middle {
    width: 60%;
  }
  .game-right {
    width: 15%;
  }
</style>