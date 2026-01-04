<script>
	import { onMount } from "svelte";

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
	$: sim = {log: "", uart_out: "", sim_out: "", mem: [], states: [{last_instruction : "", pc : -1, last_pc : -1, regs : []}, {last_instruction : "", pc : -1, last_pc : -1, regs : []}]}


	const send_request = async (task) => {
		const request_body = JSON.stringify(task)+"\r\n"+"\r\n"
		const request = new Request('http://localhost:5173/api/ar64', {
            method: 'POST',
            body: request_body
        });
		console.log("send_request: " + request_body)
        return fetch(request)
            .then(function (response) {
                console.log("response: " + response)
				console.log("response status: " + response.status)
				if (response.status == 200) {
                    return response.blob();
                }
                throw response.status;
            })
            .then(blob => blob.text())
			.then(txt => {
					console.log("response txt: " + txt)
					let o = JSON.parse(txt)
					console.log("response json: " + o.simulator_key)
					status = "OK"
					return o
				}
			)
            .catch(function (response) {
                console.log('page error: ', response)
                status = 'Server error'
        
			});
		
		//return sim;
		
		/*
		return await fetch(request)
			.then((response) => {
					console.log("aaa" + response)
					let my_json = response.json()
					console.log("bbb" + my_json)
					my_json
			})
			.then((response) => {
					console.log(response)
				}
			)*/
	}
	const handle_step = async () => {
        const task = {"action": {"name": "step"}};
		let res = await send_request(task);
		sim = res.sim
	};
	const handle_image_load = async () => {
        const task = {"action": {"name": "load image", "location": "/home/iame/Desktop/programming/non-school/ar64/image"}};
		let res = await send_request(task);
		sim = res.sim
	};
	const get_default_simulator = async () => {
        const task = {"action": {"name": "init"}};
		let res = await send_request(task);
		sim = res.sim
	};

	get_default_simulator();
	
	// Agnostic
	let status = "NO CONNECTION!"
	
	$: log  = sim.log;
	$: uart_out = String.fromCharCode(...sim.uart_out);
	$: sim_out = sim.sim_out;
	$: mem2D = gen2Dmem(sim);

	$: instruction_url = "https://luplab.gitlab.io/rvcodecjs/#q="+sim.states[0].last_instruction
	
	const bytes_per_row = 4
	function gen2Dmem(sim) {
		if (typeof sim !== 'undefined') {
			const mem2D = [];
			while(sim['mem'].length) {
				mem2D.push(sim['mem'].splice(0,bytes_per_row));
			}
			
			return mem2D;
		} 
		return []
	}

	let reg_names = ["zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0/fp", "s1", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"]
</script>


<body>	
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
	<link href="https://fonts.googleapis.com/css2?family=Cascadia+Code:ital,wght@0,200..700;1,200..700&family=Inconsolata:wght@200..900&display=swap" rel="stylesheet">
	<div class="vert">
		<div class="options">
			<button id="button_load" on:click={handle_image_load}>
				load
			</button>
			<button id="button_step" on:click={handle_step}>
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
		{#each sim.states as state, i}
			<div class="regs">
				<div class="nr">pc: {state.pc}</div>
				{#each state.regs as reg, reg_nr}
				<div class="nr"> {reg_nr}, {reg_names[reg_nr]}: {reg}  </div>
				{/each}
			</div> 
			<div class="memory">
				{#each mem2D as row, i}
					<div class="memory_row" style="color: {(i*bytes_per_row - state.pc in [0]) ? '#666': ((i*bytes_per_row - state.last_pc in [0]) ? '#afa': '#000')}">
						<div class="row_index">{(i*bytes_per_row).toString(16)}</div>
						<div class="data_row">
						{#each row as v, j}
							<div>{(v).toString(16).padStart(2,'0')}</div>
						{/each}
						</div>
					</div>
				{/each}
			</div>
		{/each}
		<div class="uart">
			{uart_out}
		</div>
		<div>
			{#key instruction_url}
			<iframe width="500" height="500" src={instruction_url}></iframe>
			{/key}
			<div class="uart">
				{sim_out}
			</div>
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
		flex-direction: column;
		border: 5px solid;
	}
	.memory_row {
		display: flex;
		flex-direction: row;
		background-color: coral;
		color: black;
		font-family: "Cascadia Code", monospace;
		font-variation-settings: "wdth" 300;
		font-size: 14px;
	}
	.data_row {
		display: flex;
		flex-direction: row;
		flex-grow: 0
	}
	.row_index {
		border-right: 1px;
		border-right: solid;
		border-right-color: black;
		padding-right: 5px;
		overflow: auto;
		flex-grow: 4
	}

	.uart {
		background-color: whitesmoke;
		width: 200px;
		border: 5px solid;
		color: 404040;
	}

</style>
sim