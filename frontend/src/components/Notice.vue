<template>
    <el-card style="max-width: 680px">
        <el-input v-model="data" style="width: 240px" placeholder="通告内容" />
        <el-button type="primary" @click="commit">发送</el-button>
    </el-card>
</template>

<script lang="ts" setup>
    import { onMounted } from 'vue';
    import { ref } from 'vue'
    import axios from 'axios'
    import { ElNotification } from 'element-plus'
    axios.defaults.baseURL = '/api'

    const data = ref('');

    const commit=()=>{
        axios.post(`/api/notice`, 
        { "data":data.value,"token": window.localStorage.getItem('easy_mc_token'),})
        .then(function (res) {
            if(res.data.msg=='success'){
                ElNotification({
                    title: 'Success',
                    message: '发送成功',
                    type: 'success',
                });
                data.value='';
            }
            else{
                ElNotification({
                    title: '发送错误',
                    message: res.data.data,
                    type: 'error',
                });
            }
        })
        .catch(function (error) {
            ElNotification({
                title: '网络错误',
                message: error,
                type: 'error',
            });
        });
    }

    onMounted(() => {
        
    });
</script>