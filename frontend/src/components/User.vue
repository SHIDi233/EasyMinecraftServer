<template>
    <div style="display: flex;width:100%">
        <div style="width:100%;margin-left: 1%;margin-right: 5%;">
            <el-card style="width: 100%;height:480px;overflow-y: auto;">
                <div style="margin-top: 40px;">
                    <el-table
                        :data="tableData"
                        style="width: 100%;"
                        border
                        v-loading="loading_2"
                    >
                        <el-table-column property="username" label="minecraft账号"/>
                        <el-table-column property="xuid" label="xuid"/>
                        <el-table-column property="role" label="角色"/>
                        <el-table-column fixed="right" >
                            <template #default="scope">
                                <el-button size="small" type="text" @click="admin(scope.$index, scope.row)">提升为管理员</el-button>
                                <el-button size="small" type="text" @click="disadmin(scope.$index, scope.row)">取消管理员</el-button>
                                <el-button size="small" type="text" @click="kick(scope.$index, scope.row)">强制下线</el-button>
                                <el-button size="small" type="text" @click="del(scope.$index, scope.row)">删除用户</el-button>
                            </template>
                        </el-table-column>
                    </el-table>
                </div>
                
            </el-card>
        </div>
    </div>
</template>

<script lang="ts" setup>
    import { ref } from 'vue';
    import { reactive } from 'vue'
    import axios from 'axios'
    axios.defaults.baseURL = '/api' 
    import { ElNotification } from 'element-plus'
    import { onMounted, computed } from 'vue';
    import type { FormInstance, FormRules } from 'element-plus'

    const loading = ref(true);

    let search = ref('')
    interface User {
        username: string
        role:String
        xuid:String     
    }
    

    const tableData = ref<User[]>([
        {
            username:'test',
            role:'test',
            xuid:'2333'
        }
    ]);

    const refresh=()=>{
        axios.get('/users')
        .then(function (res) {
            tableData.value=res.data.data;
        })
        .catch(function (error) {
            ElNotification({
                title: '网络错误',
                message: error,
                type: 'error',
            })
        });

        axios.get('/res/allowlist.json')
        .then(function (res) {
            console.log(res.data);
            for(let i=0;i<res.data.length;i++){
                for(let j=0;j<tableData.value.length;j++){
                    console.log("compare",res.data[i].name,tableData.value[j].username)
                    if(tableData.value[j].username===res.data[i].name){
                        tableData.value[j].xuid=res.data[i].xuid;
                    }
                }
            }
        })
        .catch(function (error) {
            ElNotification({
                title: '网络错误',
                message: error,
                type: 'error',
            })
        });
    };

    onMounted(() => {
        refresh();
    });

    const admin=(index,row)=>{
        axios.post(`/api/op`, 
        { "data":row.username,"token": window.localStorage.getItem('easy_mc_token'),})
        .then(function (res) {
            if(res.data.msg=='success'){
                ElNotification({
                    title: 'Success',
                    message: '发送成功',
                    type: 'success',
                });
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

    const disadmin=(index,row)=>{
        axios.delete(`/api/deop`, {data:{ "data":row.username, "token": window.localStorage.getItem('easy_mc_token'),}}
        )
        .then(function (res) {
            if(res.data.msg=='success'){
                ElNotification({
                    title: 'Success',
                    message: '发送成功',
                    type: 'success',
                });
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

    const del=(index,row)=>{
        axios.delete(`/api/del`,{data:{ "data":row.username, "token": window.localStorage.getItem('easy_mc_token'),}})
        .then(function (res) {
            if(res.data.msg=='success'){
                ElNotification({
                    title: 'Success',
                    message: '发送成功',
                    type: 'success',
                });
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

    const kick=(index,row)=>{
        axios.delete(`/api/kickoff`,{data:{ "data":row.username, "token": window.localStorage.getItem('easy_mc_token'),}})
        .then(function (res) {
            if(res.data.msg=='success'){
                ElNotification({
                    title: 'Success',
                    message: '发送成功',
                    type: 'success',
                });
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
</script>

<style>
.el-table .warning-row {
  --el-table-tr-bg-color: var(--el-color-warning-light-9);
}
.el-table .success-row {
  --el-table-tr-bg-color: var(--el-color-success-light-9);
}
</style>