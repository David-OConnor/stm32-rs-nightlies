#[doc = "Register `RCC_MP_GRSTCSETR` reader"]
pub struct R(crate::R<RCC_MP_GRSTCSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_GRSTCSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_GRSTCSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_GRSTCSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_GRSTCSETR` writer"]
pub struct W(crate::W<RCC_MP_GRSTCSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_GRSTCSETR_SPEC>;
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
impl From<crate::W<RCC_MP_GRSTCSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_GRSTCSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPSYSRST` reader - MPSYSRST"]
pub type MPSYSRST_R = crate::BitReader<bool>;
#[doc = "Field `MPSYSRST` writer - MPSYSRST"]
pub type MPSYSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_GRSTCSETR_SPEC, bool, O>;
#[doc = "Field `MCURST` reader - MCURST"]
pub type MCURST_R = crate::BitReader<bool>;
#[doc = "Field `MCURST` writer - MCURST"]
pub type MCURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_GRSTCSETR_SPEC, bool, O>;
#[doc = "Field `MPUP0RST` reader - MPUP0RST"]
pub type MPUP0RST_R = crate::BitReader<bool>;
#[doc = "Field `MPUP0RST` writer - MPUP0RST"]
pub type MPUP0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_GRSTCSETR_SPEC, bool, O>;
#[doc = "Field `MPUP1RST` reader - MPUP1RST"]
pub type MPUP1RST_R = crate::BitReader<bool>;
#[doc = "Field `MPUP1RST` writer - MPUP1RST"]
pub type MPUP1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_GRSTCSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MPSYSRST"]
    #[inline(always)]
    pub fn mpsysrst(&self) -> MPSYSRST_R {
        MPSYSRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCURST"]
    #[inline(always)]
    pub fn mcurst(&self) -> MCURST_R {
        MCURST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - MPUP0RST"]
    #[inline(always)]
    pub fn mpup0rst(&self) -> MPUP0RST_R {
        MPUP0RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MPUP1RST"]
    #[inline(always)]
    pub fn mpup1rst(&self) -> MPUP1RST_R {
        MPUP1RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPSYSRST"]
    #[inline(always)]
    #[must_use]
    pub fn mpsysrst(&mut self) -> MPSYSRST_W<0> {
        MPSYSRST_W::new(self)
    }
    #[doc = "Bit 1 - MCURST"]
    #[inline(always)]
    #[must_use]
    pub fn mcurst(&mut self) -> MCURST_W<1> {
        MCURST_W::new(self)
    }
    #[doc = "Bit 4 - MPUP0RST"]
    #[inline(always)]
    #[must_use]
    pub fn mpup0rst(&mut self) -> MPUP0RST_W<4> {
        MPUP0RST_W::new(self)
    }
    #[doc = "Bit 5 - MPUP1RST"]
    #[inline(always)]
    #[must_use]
    pub fn mpup1rst(&mut self) -> MPUP1RST_W<5> {
        MPUP1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_grstcsetr](index.html) module"]
pub struct RCC_MP_GRSTCSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_GRSTCSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_grstcsetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_GRSTCSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_grstcsetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_GRSTCSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_GRSTCSETR to value 0"]
impl crate::Resettable for RCC_MP_GRSTCSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
