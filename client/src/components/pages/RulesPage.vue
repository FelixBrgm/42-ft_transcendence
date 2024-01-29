<template>
	<div id="app">
		<GenHeader />
		<div id="Body">
		<div class="body">
			<div>
				<h1 style="font-size: 1.7em;">Official Pong Tournament Rules and Regulations</h1>
				<div>
					<p>Attention all Pong enthusiasts!</p>
					<p>Welcome to the prestigious world of competitive Pong. As we embark on this epic journey of bouncing spheres, it is imperative that participants adhere to the following rules with the utmost seriousness. Failure to comply may result in dire consequences, including but not limited to laughter from bystanders.</p>
				</div>
				<div>
				<h3 style="font-size: 1.5em;">Attire:</h3>
				<p>Players are required to don appropriate Pong attire. This includes a classic Pong-themed headband and wristbands, because nothing says serious Pong like looking like you just stepped out of a retro arcade. Failure to wear the approved attire may result in point deductions or public shaming.</p>
				</div>
				<div>
				<h3 style="font-size: 1.5em;">Pong Language:</h3>
				<p>Communication during gameplay should be conducted in the sacred language of Pong. Utilize obscure Pong-related terms to confuse and bewilder your opponents. Failure to speak in Pong dialect will result in a penalty of having to bounce the ball while hopping on one foot for an entire round.</p>
				</div>
				<div>
				<h3 style="font-size: 1.5em;">Spectator Etiquette:</h3>
				<p>Spectators are expected to maintain a solemn demeanor throughout the match. Any uncontrolled outbursts of laughter will result in immediate expulsion from the premises. Remember, Pong is serious business, and we will not tolerate any disrespect to the sanctity of the game.</p>
				</div>
				<div>
				<h3 style="font-size: 1.5em;">Paddle Enhancement Devices:</h3>
				<p>The use of paddle enhancement devices, such as motorized paddles, extra-long paddles, or paddles equipped with laser beams, is strictly prohibited. Players caught using such devices will be forced to play blindfolded for the remainder of the tournament.</p>
				</div>
				<div>
				<h3 style="font-size: 1.5em;">Victory Celebrations:</h3>
				<p>In the event of a win, players are required to execute a victory dance that incorporates classic Pong maneuvers. This may include, but is not limited to, the Paddle Spin, the Ball Bounce Boogie, or the legendary Pong Victory Shimmy. Failure to celebrate appropriately will result in a deduction of style points.</p>
				</div>
				<div>
				<h3 style="font-size: 1.5em;">Heckling Protocol:</h3>
				<p>Heckling opponents is not only allowed but encouraged. However, all heckling must be delivered in a formal and eloquent manner. Profanity and low-brow insults will not be tolerated. A well-crafted pun or a clever play on words is the preferred method of undermining your opponent's confidence.</p>
				</div>
				<div>
				<h3 style="font-size: 1.5em;">Post-Match Handshake:</h3>
				<p>Win or lose, players are required to engage in a post-match handshake. This handshake must include at least one Pong-related gesture, such as a subtle paddle twirl or a quick ball toss. Failure to execute a proper post-match handshake will result in a mandatory Pong etiquette class.</p>
				</div>
				<p style="font-size: 1.2em;">Remember, Pong is more than just a game; it's a way of life. Embrace the absurdity with a straight face, and may the Pong gods smile upon you. Good luck, and may your paddles be forever swift and your bounces be eternally unpredictable.</p>
			</div>
		</div>
		</div> 
		<GenFooter />
	</div>
	</template>
	
	<script>
	import GenHeader from "@/components/elements/GenHeader.vue";
	import GenFooter from "@/components/elements/GenFooter.vue";
	
	export default {
	components: {
		GenHeader,
		GenFooter,
	},
	};
	</script>
	
	<style>
	@import "./../functions/neonglow.css";

	.body {
		font-family: neuropol;
		padding: 1rem;
		border-radius: 20px;
		margin-left: 20px;
		margin-right: 20px;
		flex: 1;
		
		background-color: #5c5e5f;
		box-shadow: 0 0 10px 5px #00f0ff;
		animation: neonGlow 6s infinite;
		color: white;
		max-width: 1600px; /* Set the maximum width */
		margin: 0 auto; /* Center the footer */
	}
</style>
