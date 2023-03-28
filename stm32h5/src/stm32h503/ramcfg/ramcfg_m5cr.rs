#[doc = "Register `RAMCFG_M5CR` reader"]
pub struct R(crate::R<RAMCFG_M5CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMCFG_M5CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMCFG_M5CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMCFG_M5CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMCFG_M5CR` writer"]
pub struct W(crate::W<RAMCFG_M5CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMCFG_M5CR_SPEC>;
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
impl From<crate::W<RAMCFG_M5CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMCFG_M5CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCE` reader - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
pub type ECCE_R = crate::BitReader<bool>;
#[doc = "Field `ECCE` writer - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
pub type ECCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M5CR_SPEC, bool, O>;
#[doc = "Field `ALE` reader - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
pub type ALE_R = crate::BitReader<bool>;
#[doc = "Field `ALE` writer - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
pub type ALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M5CR_SPEC, bool, O>;
#[doc = "Field `SRAMER` reader - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
pub type SRAMER_R = crate::BitReader<bool>;
#[doc = "Field `SRAMER` writer - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
pub type SRAMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M5CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
    #[inline(always)]
    pub fn ecce(&self) -> ECCE_R {
        ECCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the RAMCFG_MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
    #[inline(always)]
    #[must_use]
    pub fn ecce(&mut self) -> ECCE_W<0> {
        ECCE_W::new(self)
    }
    #[doc = "Bit 4 - Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
    #[inline(always)]
    #[must_use]
    pub fn ale(&mut self) -> ALE_W<4> {
        ALE_W::new(self)
    }
    #[doc = "Bit 8 - SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
    #[inline(always)]
    #[must_use]
    pub fn sramer(&mut self) -> SRAMER_W<8> {
        SRAMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMCFG memory 5 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramcfg_m5cr](index.html) module"]
pub struct RAMCFG_M5CR_SPEC;
impl crate::RegisterSpec for RAMCFG_M5CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramcfg_m5cr::R](R) reader structure"]
impl crate::Readable for RAMCFG_M5CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramcfg_m5cr::W](W) writer structure"]
impl crate::Writable for RAMCFG_M5CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMCFG_M5CR to value 0"]
impl crate::Resettable for RAMCFG_M5CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
