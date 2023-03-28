#[doc = "Register `SAI_ACR2` reader"]
pub struct R(crate::R<SAI_ACR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_ACR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_ACR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_ACR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_ACR2` writer"]
pub struct W(crate::W<SAI_ACR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_ACR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SAI_ACR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_ACR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTH` reader - FIFO threshold. This bit is set and cleared by software."]
pub type FTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTH` writer - FIFO threshold. This bit is set and cleared by software."]
pub type FTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `FFLUSH` writer - FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled."]
pub type FFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR2_SPEC, bool, O>;
#[doc = "Field `TRIS` reader - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to for more details."]
pub type TRIS_R = crate::BitReader<bool>;
#[doc = "Field `TRIS` writer - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to for more details."]
pub type TRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR2_SPEC, bool, O>;
#[doc = "Field `MUTE` reader - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MUTE_R = crate::BitReader<bool>;
#[doc = "Field `MUTE` writer - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR2_SPEC, bool, O>;
#[doc = "Field `MUTEVAL` reader - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MUTEVAL_R = crate::BitReader<bool>;
#[doc = "Field `MUTEVAL` writer - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MUTEVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR2_SPEC, bool, O>;
#[doc = "Field `MUTECNT` reader - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to for more details."]
pub type MUTECNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUTECNT` writer - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to for more details."]
pub type MUTECNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `CPL` reader - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is �-Law algorithm or A-Law algorithm."]
pub type CPL_R = crate::BitReader<bool>;
#[doc = "Field `CPL` writer - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is �-Law algorithm or A-Law algorithm."]
pub type CPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR2_SPEC, bool, O>;
#[doc = "Field `COMP` reader - Companding mode. These bits are set and cleared by software. The �-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to for more details. Note: Companding mode is applicable only when Free protocol mode is selected."]
pub type COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP` writer - Companding mode. These bits are set and cleared by software. The �-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to for more details. Note: Companding mode is applicable only when Free protocol mode is selected."]
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2 - FIFO threshold. This bit is set and cleared by software."]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to for more details."]
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to for more details."]
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is �-Law algorithm or A-Law algorithm."]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Companding mode. These bits are set and cleared by software. The �-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to for more details. Note: Companding mode is applicable only when Free protocol mode is selected."]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FIFO threshold. This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<0> {
        FTH_W::new(self)
    }
    #[doc = "Bit 3 - FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<3> {
        FFLUSH_W::new(self)
    }
    #[doc = "Bit 4 - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to for more details."]
    #[inline(always)]
    #[must_use]
    pub fn tris(&mut self) -> TRIS_W<4> {
        TRIS_W::new(self)
    }
    #[doc = "Bit 5 - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    #[must_use]
    pub fn mute(&mut self) -> MUTE_W<5> {
        MUTE_W::new(self)
    }
    #[doc = "Bit 6 - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    #[must_use]
    pub fn muteval(&mut self) -> MUTEVAL_W<6> {
        MUTEVAL_W::new(self)
    }
    #[doc = "Bits 7:12 - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to for more details."]
    #[inline(always)]
    #[must_use]
    pub fn mutecnt(&mut self) -> MUTECNT_W<7> {
        MUTECNT_W::new(self)
    }
    #[doc = "Bit 13 - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is �-Law algorithm or A-Law algorithm."]
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CPL_W<13> {
        CPL_W::new(self)
    }
    #[doc = "Bits 14:15 - Companding mode. These bits are set and cleared by software. The �-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to for more details. Note: Companding mode is applicable only when Free protocol mode is selected."]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<14> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_acr2](index.html) module"]
pub struct SAI_ACR2_SPEC;
impl crate::RegisterSpec for SAI_ACR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_acr2::R](R) reader structure"]
impl crate::Readable for SAI_ACR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_acr2::W](W) writer structure"]
impl crate::Writable for SAI_ACR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_ACR2 to value 0"]
impl crate::Resettable for SAI_ACR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
