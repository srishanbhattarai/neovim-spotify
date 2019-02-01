" Initialize the channel for nvim-spotify
if !exists('s:spotifyjobid')
	let s:spotifyjobid = 0
endif

" Path to the binary
let s:bin = '/Users/srishanbhattarai/Documents/code/rust/nvim-spotify/target/debug/nvim-spotify'

" RPC message constants
let s:currentSong = 'current_song'

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
  echom 'stderr: ' . join(a:data, "\n")
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
		command! -nargs=0 SpotifyCurrentSong :call s:CurrentSong()
endfunction

" RPC to get the current song.
function! s:CurrentSong()
	call rpcnotify(s:spotifyjobid, s:currentSong)
endfunction

call s:init()

