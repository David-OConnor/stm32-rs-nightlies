#[doc = "Register `PWR_MPUWKUPENR` reader"]
pub struct R(crate::R<PWR_MPUWKUPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_MPUWKUPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_MPUWKUPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_MPUWKUPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_MPUWKUPENR` writer"]
pub struct W(crate::W<PWR_MPUWKUPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_MPUWKUPENR_SPEC>;
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
impl From<crate::W<PWR_MPUWKUPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_MPUWKUPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPEN1` reader - WKUPEN1"]
pub type WKUPEN1_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN1` writer - WKUPEN1"]
pub type WKUPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_MPUWKUPENR_SPEC, bool, O>;
#[doc = "Field `WKUPEN2` reader - WKUPEN2"]
pub type WKUPEN2_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN2` writer - WKUPEN2"]
pub type WKUPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_MPUWKUPENR_SPEC, bool, O>;
#[doc = "Field `WKUPEN3` reader - WKUPEN3"]
pub type WKUPEN3_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN3` writer - WKUPEN3"]
pub type WKUPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_MPUWKUPENR_SPEC, bool, O>;
#[doc = "Field `WKUPEN4` reader - WKUPEN4"]
pub type WKUPEN4_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN4` writer - WKUPEN4"]
pub type WKUPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_MPUWKUPENR_SPEC, bool, O>;
#[doc = "Field `WKUPEN5` reader - WKUPEN5"]
pub type WKUPEN5_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN5` writer - WKUPEN5"]
pub type WKUPEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_MPUWKUPENR_SPEC, bool, O>;
#[doc = "Field `WKUPEN6` reader - WKUPEN6"]
pub type WKUPEN6_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN6` writer - WKUPEN6"]
pub type WKUPEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_MPUWKUPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WKUPEN1"]
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WKUPEN2"]
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WKUPEN3"]
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUPEN4"]
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WKUPEN5"]
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WKUPEN6"]
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WKUPEN1"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen1(&mut self) -> WKUPEN1_W<0> {
        WKUPEN1_W::new(self)
    }
    #[doc = "Bit 1 - WKUPEN2"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen2(&mut self) -> WKUPEN2_W<1> {
        WKUPEN2_W::new(self)
    }
    #[doc = "Bit 2 - WKUPEN3"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen3(&mut self) -> WKUPEN3_W<2> {
        WKUPEN3_W::new(self)
    }
    #[doc = "Bit 3 - WKUPEN4"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen4(&mut self) -> WKUPEN4_W<3> {
        WKUPEN4_W::new(self)
    }
    #[doc = "Bit 4 - WKUPEN5"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen5(&mut self) -> WKUPEN5_W<4> {
        WKUPEN5_W::new(self)
    }
    #[doc = "Bit 5 - WKUPEN6"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen6(&mut self) -> WKUPEN6_W<5> {
        WKUPEN6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_mpuwkupenr](index.html) module"]
pub struct PWR_MPUWKUPENR_SPEC;
impl crate::RegisterSpec for PWR_MPUWKUPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_mpuwkupenr::R](R) reader structure"]
impl crate::Readable for PWR_MPUWKUPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_mpuwkupenr::W](W) writer structure"]
impl crate::Writable for PWR_MPUWKUPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_MPUWKUPENR to value 0"]
impl crate::Resettable for PWR_MPUWKUPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
