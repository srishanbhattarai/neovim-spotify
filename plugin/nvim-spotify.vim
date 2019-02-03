" Initialize the channel for nvim-spotify
if !exists('s:spotifyjobid')
	let s:spotifyjobid = 0
endif

" Path to the binary
let s:bin = '/Users/srishanbhattarai/Documents/code/rust/nvim-spotify/target/debug/nvim-spotify'

" RPC message constants
let s:CurrentSong = 'current_song'
let s:PlayPause = 'play_pause'
let s:Play = 'play'
let s:Pause = 'pause'
let s:Next = 'next'
let s:Previous = 'previous'

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
  command! -nargs=0 SpotifyCurrentSong :call s:rpc(s:CurrentSong)
  command! -nargs=0 SpotifyPlayPause :call s:rpc(s:PlayPause)
  command! -nargs=0 SpotifyPlay :call s:rpc(s:Play)
  command! -nargs=0 SpotifyPause :call s:rpc(s:Pause)
  command! -nargs=0 SpotifyNext :call s:rpc(s:Next)
  command! -nargs=0 SpotifyPrevious :call s:rpc(s:Previous)
endfunction

" Send an RPC message to the remote process.
function! s:rpc(rpcMessage)
	call rpcnotify(s:spotifyjobid, a:rpcMessage)
endfunction

call s:init()
