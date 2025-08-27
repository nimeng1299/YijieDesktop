use std::io::Read;

use anyhow::{bail, Result};
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum Msger {
    // 服务端推送消息
    #[strum(serialize = "updateVersion")]
    UpdateVersion, // 更新客户端版本
    #[strum(serialize = "broadcast")]
    Broadcast, // 广播通知3秒
    #[strum(serialize = "showInfoDialog")]
    ShowInfoDialog, // 通知用户显示文本信息弹窗，非阻塞。
    #[strum(serialize = "otherLogin")]
    OtherLogin, // 其他玩家登录，附带登录玩家的昵称
    #[strum(serialize = "otherLogout")]
    OtherLogout, // 其他玩家下线，附带下线玩家昵称
    #[strum(serialize = "refreshPlayerInfo")]
    RefreshPlayerInfo, // 更新玩家自己的信息
    #[strum(serialize = "refreshPlayerListSum")]
    RefreshPlayerListSum, // 更新在线玩家的数量
    #[strum(serialize = "refreshRoomList")]
    RefreshRoomList, // 更新大厅房间列表，附带房间名称，和房间里的人数信息
    #[strum(serialize = "refreshRoomInfo")]
    RefreshRoomInfo, // 更新房间信息，房间名字玩家和观众之类的
    #[strum(serialize = "refreshGameInfo")]
    RefreshGameInfo, // 更新棋盘信息，当前步数和棋子分布
    #[strum(serialize = "refreshCountdown")]
    RefreshCountdown, // 更新倒计时信息
    #[strum(serialize = "youCanMove")]
    YouCanMove, // 通知你可以行棋
    #[strum(serialize = "youNotMove")]
    YouNotMove, // 通知你不可以行棋
    #[strum(serialize = "dispatchCustomBottom")]
    DispatchCustomBottom, // 服务端分派自定义按钮
    #[strum(serialize = "dispatchAskInputDialog")]
    DispatchAskInputDialog, // 服务端向用户询问，通知客户端打开自定义文本输入弹窗 (askId&title&topTip&midTip&bottomTip&isForceInput&limitReg)
    #[strum(serialize = "gameStart")]
    GameStart, // 通知游戏开始，谁和谁在哪个房间开始游戏
    #[strum(serialize = "winMessage")]
    WinMessage, // 通知获胜信息
    #[strum(serialize = "pushNotificationIds")]
    PushNotificationIds, // 推送公告列表（仅仅只是列表id，客户端取没看过的id再像服务端请求具体类容）
    #[strum(serialize = "pushNotificationText")]
    PushNotificationText, // 根据客户端的请求，推送具体的公告内容（包含id和文本）
    #[strum(serialize = "refreshUserInfo")]
    RefreshUserInfo, // 推送个人资料
    #[strum(serialize = "pushChatMessage")]
    PushChatMessage, // 推送房间内聊天消息 （{msgId}&{nickName}&{msg}）其中msgId是连续的int可用于判断两个消息之间是否有消息没有收到，msgId始终在 000-999之间循环.
    #[strum(serialize = "refreshDefMessageList")]
    RefreshDefMessageList, // 通知用户显示预设消息文本列表（{msg};{msg}）

    // 客户端请求操作
    #[strum(serialize = "keepLive")]
    KeepLive, // 维持存活的请求，服务端不进行任何操作
    #[strum(serialize = "requestLogin")]
    RequestLogin, // 用户请求登录，附带信息 idCode@version
    #[strum(serialize = "requestRegister")]
    RequestRegister, // 用户请求登录，附带信息 idCode&nickName&contact&version
    #[strum(serialize = "requestLogout")]
    RequestLogout, // 用户请求退出
    #[strum(serialize = "requestEnterRoom")]
    RequestEnterRoom, // 用户请求进入房间
    #[strum(serialize = "requestLeaveRoom")]
    RequestLeaveRoom, // 用户离开房间
    #[strum(serialize = "requestBeChessPlayer")]
    RequestBeChessPlayer, // 申请成为下棋者，附带信息 1：黑方  2：白方
    #[strum(serialize = "requestLeaveSeat")]
    RequestLeaveSeat, // 申请离开座位
    #[strum(serialize = "requestMoveLater")]
    RequestMoveLater, // 请求落子
    #[strum(serialize = "requestAdmitDefeat")]
    RequestAdmitDefeat, // 请求认输
    #[strum(serialize = "requestChessRule")]
    RequestChessRule, // 请求详细棋规则
    #[strum(serialize = "requestChessStatistics")]
    RequestChessStatistics, // 请求棋的数据统计
    #[strum(serialize = "requestCustomBottomEvent")]
    RequestCustomBottomEvent, // 发送动态自定义按钮事件
    #[strum(serialize = "requestCacheSignContent")]
    RequestCacheSignContent, // 客户端请求获取缓存标记具体内容
    #[strum(serialize = "requestNotifications")]
    RequestNotifications, // 告诉服务端自己有哪些公告还没看过，传入公告id列表
    #[strum(serialize = "requestUserInfo")]
    RequestUserInfo, // 客户端请求获取个人资料
    #[strum(serialize = "requestUserAmountFlow")]
    RequestUserAmountFlow, // 客户端请求获取个人代币流水
    #[strum(serialize = "requestUserPlayingDetail")]
    RequestUserPlayingDetail, // 客户端请求个人对弈统计
    #[strum(serialize = "requestLoginPlayerList")]
    RequestLoginPlayerList, // 客户端请求获取登录玩家列表
    #[strum(serialize = "requestOtherPlayerInfo")]
    RequestOtherPlayerInfo, // 客户端请求查看他人信息
    #[strum(serialize = "requestRankPlayerList")]
    RequestPlayerRankList, // 客户端请求名人堂信息
    #[strum(serialize = "requestRoomRankList")]
    RequestRoomRankList, // 客户端请求热棋榜信息
    #[strum(serialize = "requestPokePlayer")]
    RequestPokePlayer, // 请求戳一戳，传入1:黑方昵称，3:黑方倒计时，2:白方昵称，4:白方倒计时
    #[strum(serialize = "requestDefMessageList")]
    RequestDefMessageList, // 请求获取预设消息列表
    #[strum(serialize = "requestSendDefMessage")]
    RequestSendDefMessage, // 请求发送预设消息，传入预设消息序号
    #[strum(serialize = "requestSendCustomMessage")]
    RequestSendCustomMessage, // 请求发送自定义文本消息
    #[strum(serialize = "requestAnswerAskInputInfo")]
    RequestAnswerAskInputInfo, // 向服务端发送自定义输入的文本，传入askId&inputText

    // 服务端返回结果
    #[strum(serialize = "connSuccess")]
    ConnSuccess, // 网络连接成功
    #[strum(serialize = "loginSuccess")]
    LoginSuccess, // 登录成功, 附带登陆者的昵称
    #[strum(serialize = "loginFailed")]
    LoginFailed, // 登录失败，附带失败原因
    #[strum(serialize = "registerSuccess")]
    RegisterSuccess, // 注册成功，附带idCode&提示信息
    #[strum(serialize = "registerFailed")]
    RegisterFailed, // 注册失败，附带失败原因
    #[strum(serialize = "enterRoomSuccess")]
    EnterRoomSuccess, // 进入房间成功
    #[strum(serialize = "enterRoomFailed")]
    EnterRoomFailed, // 进入房间失败，附带失败原因
    #[strum(serialize = "beChessPlayerSuccess")]
    BeChessPlayerSuccess, // 成为下棋者成功
    #[strum(serialize = "beChessPlayerFailed")]
    BeChessPlayerFailed, // 成为下棋者失败。附带失败原因
    #[strum(serialize = "returnCacheSignContent")]
    ReturnCacheSignContent, // 服务端返回缓存标记具体内容

    // 服务端管理员推送消息
    #[strum(serialize = "refreshAccountList")]
    RefreshAccountList, // 刷新账号列表
    #[strum(serialize = "managerRefreshRoomList")]
    ManagerRefreshRoomList, // 管理员更新房间列表
    #[strum(serialize = "refreshChessList")]
    RefreshChessList, // 展示可玩的棋类的列表
    #[strum(serialize = "refreshAccountDetail")]
    RefreshAccountDetail, // 展示账号详情
    #[strum(serialize = "refreshAmountDetail")]
    RefreshAmountDetail, // 展示资产明细
    #[strum(serialize = "refreshPlayingDetail")]
    RefreshPlayingDetail, // 展示对弈记录
    #[strum(serialize = "refreshChatAdjustDetail")]
    RefreshChatAdjustDetail, // 展示聊天权限管理详情
    #[strum(serialize = "refreshTodayRecords")]
    RefreshTodayRecords, // 展示某房间今日战报
    #[strum(serialize = "refreshRoomConfigInfo")]
    RefreshRoomConfigInfo, // 展示某房间的配置信息
    #[strum(serialize = "refreshTodayLogin")]
    RefreshTodayLogin, // 展示今日登录用户
    #[strum(serialize = "refreshSystemInfo")]
    RefreshSystemInfo, // 展示当前系统信息
    #[strum(serialize = "refreshActiveNotifications")]
    RefreshActiveNotifications, // 刷新活跃公告列表
    #[strum(serialize = "refreshOldNotificationIds")]
    RefreshOldNotificationIds, // 刷新已关闭过时公告id列表
    #[strum(serialize = "refreshOldNotificationDetails")]
    RefreshOldNotificationDetails, // 展示已关闭过时公告的内容详情

    // 管理员客户端请求操作
    #[strum(serialize = "requestManagerLogin")]
    RequestManagerLogin, // 管理员请求登陆
    #[strum(serialize = "requestManagerLogout")]
    RequestManagerLogout, // 管理员请求退出
    #[strum(serialize = "requestAccountList")]
    RequestAccountList, // 管理员请求账号列表数据
    #[strum(serialize = "requestSearchBanAccountList")]
    RequestSearchBanAccountList, // 管理员请求筛选封禁的账号列表数据
    #[strum(serialize = "requestSearchCheckAccountList")]
    RequestSearchCheckAccountList, // 管理员请求筛选审核的账号列表数据
    #[strum(serialize = "requestSearchLikeNameAccountList")]
    RequestSearchLikeNameAccountList, // 管理员请求模糊搜索昵称账号列表数据
    #[strum(serialize = "requestAccountDetail")]
    RequestAccountDetail, // 管理员请求查看用户账号详情
    #[strum(serialize = "requestAmountDetail")]
    RequestAmountDetail, // 管理员请求查看用户资产明细
    #[strum(serialize = "requestPlayingDetail")]
    RequestPlayingDetail, // 管理员请求查看用户对弈记录
    #[strum(serialize = "requestChatAdjustDetail")]
    RequestChatAdjustDetail, // 管理员请求查看用户聊天权限情况
    #[strum(serialize = "requestAddAccount")]
    RequestAddAccount, // 管理员添加账号
    #[strum(serialize = "requestRoomList")]
    RequestRoomList, // 管理员请求房间列表数据
    #[strum(serialize = "requestChessList")]
    RequestChessList, // 管理员请求可玩的棋的列表
    #[strum(serialize = "requestCreateRoom")]
    RequestCreateRoom, // 管理员请求创建房间
    #[strum(serialize = "requestCloneRoom")]
    RequestCloneRoom, // 管理员请求克隆增加房间
    #[strum(serialize = "requestCloseRoom")]
    RequestCloseRoom, // 管理员请求关闭房间
    #[strum(serialize = "requestCleanRoom")]
    RequestCleanRoom, // 管理员请求清理房间
    #[strum(serialize = "requestTodayRecords")]
    RequestTodayRecords, // 管理员请求房间今日战报
    #[strum(serialize = "requestDaysRecords")]
    RequestDaysRecords, // 管理员请求房间多日战报
    #[strum(serialize = "requestRoomConfigInfo")]
    RequestRoomConfigInfo, // 管理员请求查看某个房间的配置详情
    #[strum(serialize = "requestTodayLogin")]
    RequestTodayLogin, // 管理员请求查看今日登录用户
    #[strum(serialize = "requestSystemInfo")]
    RequestSystemInfo, // 管理员请求查看系统资源信息
    #[strum(serialize = "requestGroupNotice")]
    RequestGroupNotice, // 管理员请求给所有在线用户发一个消息
    #[strum(serialize = "requestAddNotification")]
    RequestAddNotification, // 管理员创建一封公告
    #[strum(serialize = "requestCloseNotification")]
    RequestCloseNotification, // 管理员关闭一封公告
    #[strum(serialize = "requestActiveNotifications")]
    RequestActiveNotifications, // 管理员获取所有活跃的公告列表，包含id和内容
    #[strum(serialize = "requestOldNotificationIds")]
    RequestOldNotificationIds, // 获取过时的公告id列表
    #[strum(serialize = "requestOldNotificationDetail")]
    RequestOldNotificationDetail, // 根据id获取过时的公告内容
    #[strum(serialize = "requestPassAccount")]
    RequestPassAccount, // 管理员审核通过账号
    #[strum(serialize = "requestRejectAccount")]
    RequestRejectAccount, // 管理员审核拒绝账号
    #[strum(serialize = "requestChangeAmount")]
    RequestChangeAmount, // 管理员申请增加或者减少逆界代币额度
    #[strum(serialize = "requestChangeBadge")]
    RequestChangeBadge, // 管理员申请修改荣誉标签
    #[strum(serialize = "requestChangeContact")]
    RequestChangeContact, // 管理员申请修改联系方式
    #[strum(serialize = "requestAdjustChatLevel")]
    RequestAdjustChatLevel, // 管理员申请调整聊天权限

    // 管理员服务端返回结果
    #[strum(serialize = "managerLoginSuccess")]
    ManagerLoginSuccess, // 管理员登录成功
    #[strum(serialize = "managerLoginFailed")]
    ManagerLoginFailed, // 管理员登录失败
    #[strum(serialize = "managerAddAccountFailed")]
    ManagerAddAccountFailed, // 管理员添加账号失败
    #[strum(serialize = "managerAddAccountSuccess")]
    ManagerAddAccountSuccess, // 管理员添加账号成功
}

impl Msger {
    pub fn parse(msg: String) -> Result<(Self, String)> {
        let msgs: Vec<&str> = msg.splitn(2, '=').collect();
        if msgs.len() == 2 {
            let m: Msger = msgs[0].parse()?;
            Ok((m, msgs[1].to_string()))
        } else {
            bail!("msg format error")
        }
    }

    pub fn to_msg(&self, msg: String) -> Vec<u8> {
        write_utf(format!("{}={}", self, msg))
    }
}

fn write_utf(message: String) -> Vec<u8> {
    let msg_bytes = message.as_bytes();
    let len = msg_bytes.len();

    // 确保长度不超过Java允许的最大值(65535)
    if len > 65535 {
        panic!("Message too long for UTF encoding");
    }

    let mut result = Vec::with_capacity(len + 2);

    // 添加两个字节的长度信息(大端序)
    result.push((len >> 8) as u8);
    result.push((len & 0xFF) as u8);

    // 添加消息内容
    result.extend_from_slice(msg_bytes);

    result
}
/// 从 Java writeUTF 格式解码字符串
pub fn read_utf(reader: &mut &[u8]) -> Result<Vec<String>> {
    let mut results = Vec::new();
    let mut first = 0;
    loop {
        let (utf8_len, s) = read_utf_first(&mut &reader[first..])?;
        results.push(s);
        first += utf8_len as usize;
        if utf8_len == 0  || first >= reader.len() - 2 {
            break;
        }
    }
    
    Ok(results)
}

fn read_utf_first(reader: &mut &[u8]) -> Result<(u32, String)> {

    // 读取长度前缀（大端序）
    let mut len_bytes = [0u8; 2];
    reader.read_exact(&mut len_bytes)?;
    let utf8_len = u16::from_be_bytes(len_bytes) as usize;
    
    // 读取 UTF-8 编码数据
    let mut utf8_bytes = vec![0u8; utf8_len];
    reader.read_exact(&mut utf8_bytes)?;
    
    // 解码修改版 UTF-8
    let mut result = String::new();
    let mut i = 0;
    
    while i < utf8_len {
        let byte1 = utf8_bytes[i];
        
        // 单字节 ASCII (0xxxxxxx)
        if byte1 & 0x80 == 0 {
            if byte1 == 0 {
                // 标准 UTF-8 中不应出现单字节 0，但在 Java 修改版中可能
                result.push('\0');
            } else {
                result.push(byte1 as char);
            }
            i += 1;
        }
        // 双字节序列 (110xxxxx)
        else if byte1 & 0xE0 == 0xC0 && i + 1 < utf8_len {
            let byte2 = utf8_bytes[i + 1];
            
            // 检查是否为 Java 修改版 UTF-8 的 null 字符
            if byte1 == 0xC0 && byte2 == 0x80 {
                result.push('\0');
                i += 2;
                continue;
            }
            
            // 常规双字节字符
            if byte2 & 0xC0 == 0x80 {
                let code_point = ((byte1 & 0x1F) as u16) << 6 | (byte2 & 0x3F) as u16;
                if let Some(c) = char::from_u32(code_point as u32) {
                    result.push(c);
                } else {
                    bail!("Invalid UTF-8 sequence");
                }
                i += 2;
            } else {
                 bail!("Invalid UTF-8 sequence");
            }
        }
        // 三字节序列 (1110xxxx)
        else if byte1 & 0xF0 == 0xE0 && i + 2 < utf8_len {
            let byte2 = utf8_bytes[i + 1];
            let byte3 = utf8_bytes[i + 2];
            
            if byte2 & 0xC0 == 0x80 && byte3 & 0xC0 == 0x80 {
                let code_point = ((byte1 & 0x0F) as u32) << 12 
                    | ((byte2 & 0x3F) as u32) << 6 
                    | (byte3 & 0x3F) as u32;
                
                if let Some(c) = char::from_u32(code_point) {
                    result.push(c);
                } else {
                    bail!("Invalid UTF-8 sequence");
                }
                i += 3;
            } else {
                 bail!("Invalid UTF-8 sequence");
            }
        }
        else {
            bail!("Invalid UTF-8 sequence");
        }
    }
    
    Ok(((utf8_len + 2) as u32, result))
}