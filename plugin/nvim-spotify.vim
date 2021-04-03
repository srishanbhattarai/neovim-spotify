" Initialize the channel for nvim-spotify
if !exists('s:spotifyjobid')
	let s:spotifyjobid = 0
endif

" Path to the binary
let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = s:scriptdir . '/target/release/nvim-spotify'

" RPC message constants
let s:SpotifyCurrentSong = 'spotify_current_song'
let s:SpotifyPlayPause = 'spotify_play_pause'
let s:SpotifyPlay = 'spotify_play'
let s:SpotifyPause = 'spotify_pause'
let s:SpotifyNext = 'spotify_next'
let s:SpotifyPrevious = 'spotify_previous'
let s:SpotifyLyrics = 'spotify_lyrics'

let s:AppleMusicCurrentSong = 'applemusic_current_song'
let s:AppleMusicPlayPause = 'applemusic_play_pause'
let s:AppleMusicPlay = 'applemusic_play'
let s:AppleMusicPause = 'applemusic_pause'
let s:AppleMusicNext = 'applemusic_next'
let s:AppleMusicPrevious = 'applemusic_previous'
let s:AppleMusicLyrics = 'applemusic_lyrics'

" Entry point
function! s:init()
  call s:connect()
endfunction

" Get the Job ID and check for errors. If no errors, attach RPC handlers to
" the commands.
function! s:connect()
  let jobID = s:GetJobID()

  if 0 == jobID
    echoerr "spotify: cannot start rpc process"
  elseif -1 == jobID
    echoerr "spotify: rpc process is not executable"
  else
    let s:spotifyjobid = jobID
    call s:AttachRPCHandlers(jobID)
  endif
endfunction

" Function reference in case of RPC start errors
function! s:OnStderr(id, data, event) dict
  echom 'stderr: ' . a:event . join(a:data, "\n") 
endfunction

" Start the RPC job and return the job  (channel) ID
function! s:GetJobID()
  if s:spotifyjobid == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true, 'on_stderr': function('s:OnStderr') })
    return jobid
  else
    return s:spotifyjobid
  endif
endfunction

" Associate commands with their RPC invocations
function! s:AttachRPCHandlers(jobID)
  command! -nargs=0 SpotifyCurrentSong :call s:rpc(s:SpotifyCurrentSong)
  command! -nargs=0 SpotifyPlayPause :call s:rpc(s:SpotifyPlayPause)
  command! -nargs=0 SpotifyPlay :call s:rpc(s:SpotifyPlay)
  command! -nargs=0 SpotifyPause :call s:rpc(s:SpotifyPause)
  command! -nargs=0 SpotifyNext :call s:rpc(s:SpotifyNext)
  command! -nargs=0 SpotifyPrevious :call s:rpc(s:SpotifyPrevious)
  command! -nargs=0 SpotifyLyrics :call s:rpc(s:SpotifyLyrics)

  command! -nargs=0 AppleMusicCurrentSong :call s:rpc(s:AppleMusicCurrentSong)
  command! -nargs=0 AppleMusicPlayPause :call s:rpc(s:AppleMusicPlayPause)
  command! -nargs=0 AppleMusicPlay :call s:rpc(s:AppleMusicPlay)
  command! -nargs=0 AppleMusicPause :call s:rpc(s:AppleMusicPause)
  command! -nargs=0 AppleMusicNext :call s:rpc(s:AppleMusicNext)
  command! -nargs=0 AppleMusicPrevious :call s:rpc(s:AppleMusicPrevious)
  command! -nargs=0 AppleMusicLyrics :call s:rpc(s:AppleMusicLyrics)
endfunction

" Send an RPC message to the remote process.
function! s:rpc(rpcMessage)
	call rpcnotify(s:spotifyjobid, a:rpcMessage)
endfunction

call s:init()
