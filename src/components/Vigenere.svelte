<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	let message = "";
	let key = "";
	let encode = true;
	let output = "";

	async function vigenere() {
		output = await invoke("vigenere", { message, key, encode });
	}
</script>

<section>
	<div class="card m-2">
		<label for="message" class="ps-2 pt-2">Message</label>
		<div class="input-group p-2">
			<input
				type="text"
				class="form-control"
				id="message"
				placeholder="Enter a message..."
				bind:value={message}
			/>
			<button
				class="btn btn-outline-secondary dropdown-toggle"
				type="button"
				data-bs-toggle="dropdown"
				aria-expanded="false">{encode ? "Encoding" : "Decoding"}</button
			>
			<ul class="dropdown-menu">
				<li><button class="dropdown-item" on:click={() => (encode = true)}>Encoding</button></li>
				<li><button class="dropdown-item" on:click={() => (encode = false)}>Decoding</button></li>
			</ul>
		</div>
		<label for="key" class="ps-2">Key</label>
		<div class="input-group mb-2 p-2">
			<input
				type="text"
				class="form-control"
				id="key"
				placeholder="Enter a key..."
				bind:value={key}
			/>
			<button class="btn btn-outline-secondary" type="button" id="button-addon2" on:click={vigenere}
				>Generate</button
			>
		</div>

		<div class="card-footer">
			{output ? "Output: " + output : "Nothing Inputted!"}
		</div>
	</div>
</section>
