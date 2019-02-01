if !exists('s:spotifyjobid')
	let s:spotifyjobid = 0
endif

let s:bin = '/Users/srishanbhattarai/Documents/code/rust/nvim-spotify/target/debug/nvim-spotify'

function! s:init()
  call s:connect()
endfunction

function! s:connect()
	let jobID = s:GetJobID()

  if 0 == jobID
    echoerr "spotify: cannot start rpc process"
  elseif -1 == jobID
    echoerr "spotify: rpc process is not executable"
  else
    let s:spotifyjobid = jobID
    call s:ConfigureJob(jobID)
  endif
endfunction

function! s:ConfigureJob(jobID)
		command! -nargs=1 Spotify  :call s:DefaultCommand(<f-args>)
endfunction

function! s:DefaultCommand(args)
	call rpcnotify(s:spotifyjobid, a:args)
endfunction

function! s:OnStderr(id, data, event) dict
  echom 'scorched earth: stderr: ' . join(a:data, "\n")
endfunction

function! s:GetJobID()
	if s:spotifyjobid == 0
		let jobid = jobstart([s:bin], { 'rpc': v:true, 'on_stderr': function('s:OnStderr') })
		return jobid
	else
		return s:spotifyjobid
	endif
endfunction

call s:init()

