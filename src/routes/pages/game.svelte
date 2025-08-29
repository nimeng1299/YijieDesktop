<script>
    import GameBoard from "./GameBoard.svelte";
    import GameCard from "./GameCard.svelte";

      let {tabId, room, game} = $props();
</script>
<div class="game-container ">
  <div class="game-left">
    <div class="gamecard">
      <GameCard player={room.black_player} score={game.black_score} />
    </div>
    <div class="gamecard">
      <GameCard player={room.white_player} score={game.white_score} />
    </div>
  </div>
  <div class="game-middle">
    <GameBoard game={game} />
  </div>
  <div class="game-right">右侧</div>
</div>

<style>
  .game-container {
    display: flex;
    height: 90vh; /* 添加高度以便查看效果 */
    gap: 10px;
  }
  .game-left {
    width: 25%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }
  .gamecard {
    width: 100%; /* 添加宽度以占满 game-left */
  }
  .game-middle {
    width: 60%;
  }
  .game-right {
    width: 15%;
  }
</style>