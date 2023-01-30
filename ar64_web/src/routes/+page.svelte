<script>
	/* WASM
	export let bindings;
	function handleStep(){
		str = bindings.step_json_simulator(str)
	}
	function load_img(){
		str = bindings.load_image()
	}
	let str = bindings.get_default_simulator();
	*/

	// TCP
	const send_request = (task) => {
		const request = new Request('http://127.0.0.1:5173/api/ar64', {
            method: 'PATCH',
            body: JSON.stringify(task)
        });
        fetch(request)
            .then(function (response) {
                if (response.status == 200) {
                    return response.blob();
                }
                throw response.status;
            })
            .then(blob => blob.text())
            
            .then(function(response) {
				//.then(txt => JSON.parse(txt))
                // do something with the data sent in the request
                console.log('page: got a response' + response + " end of response")
                if (response["error"] != undefined) {
                    console.log('page: ' + response["error"])
                } else {
                    status = 'OK'
                }
            })
            .catch(function (response) {
                console.log('page error: ', response)
                status = 'Server error'
            });
	}
	const handle_step = () => {
        const task = {"action": "step"};

		send_request(task)
	};
	const handle_image_load = () => {
        const task = {"action": "load image"};
		send_request(task)
	};
	const get_default_simulator = () => {
        const task = {"action": "init"};
		send_request(task)
	};

	let str = get_default_simulator();
	
	// Agnostic
	//$: sim = JSON.parse(str);
	let status = "NO CONNECTION!"
	$: sim = {log: "", uart_out: "", mem: [], pc: -1, states: []}
	$: log  = sim.log;
	$: uart_out = sim.uart_out;
	$: mem2D = gen2Dmem(sim);

	function gen2Dmem(sim) {
		if (typeof sim !== 'undefined') {
			const mem2D = [];
			while(sim.mem.length) mem2D.push(sim.mem.splice(0,8));
			
			return mem2D;
		} 
		return []
	}


</script>


<body>	
	<div class="vert">
		<div class="options">
			<button on:click={handle_image_load}>
				load image
			</button>
			<button on:click={handle_step}>
				step
			</button>
			<div class="log">
				#cores: {sim.states.length}
			</div>
			<div class="log">
				 - sim: {log}
			</div>
			<div class="log">
				- server: {status}
		   </div>

		</div>
		<div class="global">
		{#each sim.states as state}
			<div class="regs">
				<div class="nr">pc: {state.pc}</div>
				{#each state.regs as reg, i}
				<div class="nr">{i}: {reg}</div>
				{/each}
			</div> 
			<div class="memory">
				<div class="row-index">
					{#each mem2D as row, i}
						<div>{i*8}</div>
					{/each}
				</div>
				<div>
				{#each mem2D as row, i}
					<div class="row">
					{#each row as v, j}
						<div style="color: {i*8 + j - state.pc in [0, 1, 2, 3] ? '#333': '#ccc'}">{(v).toString(16).padStart(2,'0')}</div>
					{/each}
					</div>
				{/each}
				</div>
			</div>

		{/each}
		<div class="uart">
			{uart_out}
		</div>
	</div>
	</div>
</body>



<style>
	body {
		background-color: black;
		padding: 0px;
	}
	.global {
		display: flex;
	}
	.options{
		display: flex;
		flex-direction: row;
	}
	.vert {
		flex-direction: column;
	}
	.regs {
		display: flex;
		border: 5px solid;
		color: 404040;
		display: flex;
		flex-direction: column;
		font-size: 17px;
		min-width: 3em;;
	}
	.nr {
		color: coral;
	}
	.log {
		color: coral;
		vertical-align: middle;
		display: inline-block;
		line-height: 2;
	}
	.memory {
		display: flex;
		flex-direction: row;
		border: 5px solid;
	}
	.row {
		display: flex;
		flex-direction: row;
		color: coral;
		font-family: monospace;
		font-size: 17px;
	}
	.row-index {
		border-right: 10px;
		border-right: solid;
		border-right-color: black;
		background-color: coral;
		color: black;
		padding-right: 5px;
		overflow: auto;
		width: 10;
		font-family: monospace;
		font-size: 17px;
	}

	.uart {
		background-color: whitesmoke;
		width: 600px;
		border: 5px solid;
		color: 404040;
	}

</style>
