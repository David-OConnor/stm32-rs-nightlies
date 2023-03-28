#[doc = "Register `SYSCFG_CMPENCLRR` reader"]
pub struct R(crate::R<SYSCFG_CMPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_CMPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_CMPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_CMPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_CMPENCLRR` writer"]
pub struct W(crate::W<SYSCFG_CMPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_CMPENCLRR_SPEC>;
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
impl From<crate::W<SYSCFG_CMPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_CMPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPU_EN` reader - MPU_EN"]
pub type MPU_EN_R = crate::BitReader<bool>;
#[doc = "Field `MPU_EN` writer - MPU_EN"]
pub type MPU_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_CMPENCLRR_SPEC, bool, O>;
#[doc = "Field `MCU_EN` reader - MCU_EN"]
pub type MCU_EN_R = crate::BitReader<bool>;
#[doc = "Field `MCU_EN` writer - MCU_EN"]
pub type MCU_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_CMPENCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MPU_EN"]
    #[inline(always)]
    pub fn mpu_en(&self) -> MPU_EN_R {
        MPU_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCU_EN"]
    #[inline(always)]
    pub fn mcu_en(&self) -> MCU_EN_R {
        MCU_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU_EN"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_en(&mut self) -> MPU_EN_W<0> {
        MPU_EN_W::new(self)
    }
    #[doc = "Bit 1 - MCU_EN"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_en(&mut self) -> MCU_EN_W<1> {
        MCU_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG compensation cell enable set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cmpenclrr](index.html) module"]
pub struct SYSCFG_CMPENCLRR_SPEC;
impl crate::RegisterSpec for SYSCFG_CMPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_cmpenclrr::R](R) reader structure"]
impl crate::Readable for SYSCFG_CMPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_cmpenclrr::W](W) writer structure"]
impl crate::Writable for SYSCFG_CMPENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_CMPENCLRR to value 0"]
impl crate::Resettable for SYSCFG_CMPENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
