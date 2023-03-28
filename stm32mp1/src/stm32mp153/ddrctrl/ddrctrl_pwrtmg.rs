#[doc = "Register `DDRCTRL_PWRTMG` reader"]
pub struct R(crate::R<DDRCTRL_PWRTMG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PWRTMG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PWRTMG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PWRTMG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PWRTMG` writer"]
pub struct W(crate::W<DDRCTRL_PWRTMG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PWRTMG_SPEC>;
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
impl From<crate::W<DDRCTRL_PWRTMG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PWRTMG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWERDOWN_TO_X32` reader - POWERDOWN_TO_X32"]
pub type POWERDOWN_TO_X32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POWERDOWN_TO_X32` writer - POWERDOWN_TO_X32"]
pub type POWERDOWN_TO_X32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PWRTMG_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_DPD_X4096` reader - T_DPD_X4096"]
pub type T_DPD_X4096_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_DPD_X4096` writer - T_DPD_X4096"]
pub type T_DPD_X4096_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PWRTMG_SPEC, u8, u8, 8, O>;
#[doc = "Field `SELFREF_TO_X32` reader - SELFREF_TO_X32"]
pub type SELFREF_TO_X32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELFREF_TO_X32` writer - SELFREF_TO_X32"]
pub type SELFREF_TO_X32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PWRTMG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:4 - POWERDOWN_TO_X32"]
    #[inline(always)]
    pub fn powerdown_to_x32(&self) -> POWERDOWN_TO_X32_R {
        POWERDOWN_TO_X32_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - T_DPD_X4096"]
    #[inline(always)]
    pub fn t_dpd_x4096(&self) -> T_DPD_X4096_R {
        T_DPD_X4096_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SELFREF_TO_X32"]
    #[inline(always)]
    pub fn selfref_to_x32(&self) -> SELFREF_TO_X32_R {
        SELFREF_TO_X32_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - POWERDOWN_TO_X32"]
    #[inline(always)]
    #[must_use]
    pub fn powerdown_to_x32(&mut self) -> POWERDOWN_TO_X32_W<0> {
        POWERDOWN_TO_X32_W::new(self)
    }
    #[doc = "Bits 8:15 - T_DPD_X4096"]
    #[inline(always)]
    #[must_use]
    pub fn t_dpd_x4096(&mut self) -> T_DPD_X4096_W<8> {
        T_DPD_X4096_W::new(self)
    }
    #[doc = "Bits 16:23 - SELFREF_TO_X32"]
    #[inline(always)]
    #[must_use]
    pub fn selfref_to_x32(&mut self) -> SELFREF_TO_X32_W<16> {
        SELFREF_TO_X32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL low power timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pwrtmg](index.html) module"]
pub struct DDRCTRL_PWRTMG_SPEC;
impl crate::RegisterSpec for DDRCTRL_PWRTMG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pwrtmg::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PWRTMG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pwrtmg::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PWRTMG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PWRTMG to value 0x0040_2010"]
impl crate::Resettable for DDRCTRL_PWRTMG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_2010;
}
