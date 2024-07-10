<script lang="ts" setup>
</script>

<template>
    <form>

        <fieldset>

            <legend>Assessment Information</legend>
            <div class="assessment-inputs">
                <!-- <div class="assessor-input horizontal-input"> -->
                <div class="assessor-input">
                    <p>Assessor</p>
                    <div class="horizontal-input input-border">
                        <input type="radio" id="html" name="fav_language" value="HTML">
                        <label for="html">HTML</label><br>
                        <input type="radio" id="css" name="fav_language" value="CSS">
                        <label for="css">CSS</label><br>
                        <input type="radio" id="javascript" name="fav_language" value="JavaScript">
                        <label for="javascript">JavaScript</label> 
                    </div>
                </div>
                <div class="assessment-type-input">
                    <p>Type</p>
                    <div class="checkboxes">
                        <span>
                            <input type="checkbox" id="html2" name="fav_language" value="HTML">
                            <label for="html2">HTML</label><br>
                        </span>
                        <span>
                            <input type="checkbox" id="css2" name="fav_language" value="CSS">
                            <label for="css2">CSS</label><br>
                        </span>
                        <span>
                            <input type="checkbox" id="html2" name="fav_language" value="HTML">
                            <label for="html2">HTML</label><br>
                        </span>
                        <span>
                            <input type="checkbox" id="css2" name="fav_language" value="CSS">
                            <label for="css1">CSS</label><br>
                        </span>
                    </div>
                </div>
                <div class="company-input">
                    <p>Company</p>
                </div>
                <div class="date-of-assessment-input">
                    <p>DOA</p>
                    <span>
                        <label for="date-input">Date of Assessment</label>
                        <input id="date-input" type="text" name="doa" />
                    </span>
                </div>
            </div>

        </fieldset>

        <fieldset>
            <legend>Client Information</legend>
        </fieldset>

        <fieldset>
            <legend>Insurance Information</legend>
        </fieldset>

    </form>
</template>

<style lang="scss" scoped>

    @use '../variables';

    .assessment-inputs {

        display: grid;
        grid-template-columns: repeat(2, 1fr);
        grid-template-rows: repeat(3, 1fr);
        grid-column-gap: 0px;
        grid-row-gap: 5px;

        .assessor-input { grid-area: 1 / 1 / 2 / 3; }
        .assessment-type-input { grid-area: 2 / 1 / 3 / 2; }
        .company-input { grid-area: 2 / 2 / 3 / 3; }
        .date-of-assessment-input { grid-area: 3 / 1 / 4 / 2; }

        height: 100%;
        margin: 0;

    }

    .horizontal-input {

        display: flex;
        flex-direction: row;
        column-gap: 1rem;
        align-items: center;
        flex-wrap: wrap;

    }

    .checkboxes {
      //width: 60px;
      display: grid;
      grid-column-gap: 1rem;
      grid-row-gap: 0.5rem;
      grid-template-columns: 1fr 1fr 1fr;
    }

    .input-border {

        border: 2px solid variables.$input-border-color;
        border-radius: 4px;
        background-color: variables.$accent-color;
        padding: 2px;

    }

</style>
