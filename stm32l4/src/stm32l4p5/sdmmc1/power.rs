#[doc = "Register `POWER` reader"]
pub struct R(crate::R<POWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER` writer"]
pub struct W(crate::W<POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_SPEC>;
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
impl From<crate::W<POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRCTRL` reader - SDMMC state control bits"]
pub type PWRCTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRCTRL` writer - SDMMC state control bits"]
pub type PWRCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POWER_SPEC, u8, u8, 2, O>;
#[doc = "Field `VSWITCH` reader - Voltage switch sequence start"]
pub type VSWITCH_R = crate::BitReader<bool>;
#[doc = "Field `VSWITCH` writer - Voltage switch sequence start"]
pub type VSWITCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, bool, O>;
#[doc = "Field `VSWITCHEN` reader - Voltage switch procedure enable"]
pub type VSWITCHEN_R = crate::BitReader<bool>;
#[doc = "Field `VSWITCHEN` writer - Voltage switch procedure enable"]
pub type VSWITCHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, bool, O>;
#[doc = "Field `DIRPOL` reader - Data and command direction signals polarity selection"]
pub type DIRPOL_R = crate::BitReader<bool>;
#[doc = "Field `DIRPOL` writer - Data and command direction signals polarity selection"]
pub type DIRPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - SDMMC state control bits"]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Voltage switch sequence start"]
    #[inline(always)]
    pub fn vswitch(&self) -> VSWITCH_R {
        VSWITCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage switch procedure enable"]
    #[inline(always)]
    pub fn vswitchen(&self) -> VSWITCHEN_R {
        VSWITCHEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data and command direction signals polarity selection"]
    #[inline(always)]
    pub fn dirpol(&self) -> DIRPOL_R {
        DIRPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDMMC state control bits"]
    #[inline(always)]
    #[must_use]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<0> {
        PWRCTRL_W::new(self)
    }
    #[doc = "Bit 2 - Voltage switch sequence start"]
    #[inline(always)]
    #[must_use]
    pub fn vswitch(&mut self) -> VSWITCH_W<2> {
        VSWITCH_W::new(self)
    }
    #[doc = "Bit 3 - Voltage switch procedure enable"]
    #[inline(always)]
    #[must_use]
    pub fn vswitchen(&mut self) -> VSWITCHEN_W<3> {
        VSWITCHEN_W::new(self)
    }
    #[doc = "Bit 4 - Data and command direction signals polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn dirpol(&mut self) -> DIRPOL_W<4> {
        DIRPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](index.html) module"]
pub struct POWER_SPEC;
impl crate::RegisterSpec for POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power::R](R) reader structure"]
impl crate::Readable for POWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power::W](W) writer structure"]
impl crate::Writable for POWER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
