<script>
  import { invoke } from "@tauri-apps/api/core";
  // 接收传递的数据
  let { tab_id, datas, account } = $props();

  // 处理玩家信息
  function append_player(data) {
    let str = "";
    if (data.black_player === "-1") {
      str += "暂无";
    } else {
      str += data.black_player;
    }
    str += "\tvs ";
    if (data.white_player === "-1") {
      str += "暂无";
    } else {
      str += data.white_player;
    }
    str += "\t";
    str += data.player_num;
    str += " / ";
    str += data.max_player_num;
    return str;
  }
  
  function enter_room(room_name) {
    invoke("request_enter_room", {tabId: tab_id, roomName: room_name});
  }
</script>

<div class="roomlist-container border-solid">
  <div class="roomlist-layout">
    <div class="left-section">
      <!-- 左侧内容区域 -->
      <ul class="list bg-base-100 rounded-box shadow-md">
        <li class="p-4 pb-2 text-xs opacity-60 tracking-wide">房间列表</li>
        {#each datas.rooms as data}
          <li class="list-row">
            <div>
              <div>{data.name}</div>
              <div class="text-xs uppercase font-semibold opacity-60">
                {append_player(data)}
              </div>
            </div>
            <p class="list-col-wrap text-xs">
              {data.introduction}
            </p>
            <div class="flex justify-end">
              <button class="btn btn-square btn-ghost" on:click={() => enter_room(data.name)}>
                <svg
                  class="size-[1.2em]"
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  ><g
                    stroke-linejoin="round"
                    stroke-linecap="round"
                    stroke-width="2"
                    fill="none"
                    stroke="currentColor"
                    ><path d="M6 3L20 12 6 21 6 3z"></path></g
                  ></svg
                >
              </button>
            </div>
          </li>
        {/each}
      </ul>
    </div>
    <div class="right-section"  style="padding-left: 10px;">
      <!-- 右侧内容区域 -->
      <div class="card card-dash shadow-sm">
        <div class="card-body">
          <h2 class="card-title">{account.nick_name}</h2>
          <p>
            {account.other_user_info}
          </p>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .roomlist-layout {
    display: flex;
  }
  .left-section {
    width: 65%;
    max-height: 90vh; /* 设置最大高度 */
    overflow-y: auto; /* 添加垂直滚动条 */
  }
  .right-section {
    width: 35%;
  }

  .roomlist-container {
    padding: 0px;
    width: 100%;
  }
</style>
