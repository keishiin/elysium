interface SignInResponse {
    id: string;
    username: string;
    email: string;
    steam_id?: string;
    psn_id?: string;
    xbox_id?: string;
  }

  interface CustomError {
    error: string,
  }

interface PlayerInfo {
  steamid: string,
  communityvisibilitystate: number,
  profilestate: number,
  personaname: string,
  lastlogoff?: number,
  commentpermission?: number,
  profileurl: string,
  avatar: string,
  avatarhash: string,
  avatarmedium: string,
  avatarfull: string,
  personastate?: number,
  realname?: string,
  primaryclanid: string,
  timecreated?: number,
  personastateflags: number,
  gameextrainfo: string,
  gameid?: string,
  loccountrycode?: string,
  locstatecode?: string,
   loccityid?: number,
}

interface Game {
  app_id: number,
  name?: string,
  recent_play_time?: number,
  total_play_time?: number,
  img_icon_url?: string,
  img_logo_url?: string,
  has_community_visible_stats?: boolean
}