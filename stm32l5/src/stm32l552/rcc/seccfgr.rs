#[doc = "Register `SECCFGR` reader"]
pub struct R(crate::R<SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSISEC` reader - HSISEC"]
pub type HSISEC_R = crate::BitReader<bool>;
#[doc = "Field `HSISEC` writer - HSISEC"]
pub type HSISEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `HSESEC` reader - HSESEC"]
pub type HSESEC_R = crate::BitReader<bool>;
#[doc = "Field `HSESEC` writer - HSESEC"]
pub type HSESEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `MSISEC` reader - MSISEC"]
pub type MSISEC_R = crate::BitReader<bool>;
#[doc = "Field `MSISEC` writer - MSISEC"]
pub type MSISEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `LSISEC` reader - LSISEC"]
pub type LSISEC_R = crate::BitReader<bool>;
#[doc = "Field `LSISEC` writer - LSISEC"]
pub type LSISEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `LSESEC` reader - LSESEC"]
pub type LSESEC_R = crate::BitReader<bool>;
#[doc = "Field `LSESEC` writer - LSESEC"]
pub type LSESEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SYSCLKSEC` reader - SYSCLKSEC"]
pub type SYSCLKSEC_R = crate::BitReader<bool>;
#[doc = "Field `SYSCLKSEC` writer - SYSCLKSEC"]
pub type SYSCLKSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `PRESCSEC` reader - PRESCSEC"]
pub type PRESCSEC_R = crate::BitReader<bool>;
#[doc = "Field `PRESCSEC` writer - PRESCSEC"]
pub type PRESCSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `PLLSEC` reader - PLLSEC"]
pub type PLLSEC_R = crate::BitReader<bool>;
#[doc = "Field `PLLSEC` writer - PLLSEC"]
pub type PLLSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `PLLSAI1SEC` reader - PLLSAI1SEC"]
pub type PLLSAI1SEC_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI1SEC` writer - PLLSAI1SEC"]
pub type PLLSAI1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `PLLSAI2SEC` reader - PLLSAI2SEC"]
pub type PLLSAI2SEC_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI2SEC` writer - PLLSAI2SEC"]
pub type PLLSAI2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `CLK48MSEC` reader - CLK48MSEC"]
pub type CLK48MSEC_R = crate::BitReader<bool>;
#[doc = "Field `CLK48MSEC` writer - CLK48MSEC"]
pub type CLK48MSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `HSI48SEC` reader - HSI48SEC"]
pub type HSI48SEC_R = crate::BitReader<bool>;
#[doc = "Field `HSI48SEC` writer - HSI48SEC"]
pub type HSI48SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `RMVFSEC` reader - RMVFSEC"]
pub type RMVFSEC_R = crate::BitReader<bool>;
#[doc = "Field `RMVFSEC` writer - RMVFSEC"]
pub type RMVFSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HSISEC"]
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSESEC"]
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSISEC"]
    #[inline(always)]
    pub fn msisec(&self) -> MSISEC_R {
        MSISEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSISEC"]
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LSESEC"]
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYSCLKSEC"]
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PRESCSEC"]
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSEC"]
    #[inline(always)]
    pub fn pllsec(&self) -> PLLSEC_R {
        PLLSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLLSAI1SEC"]
    #[inline(always)]
    pub fn pllsai1sec(&self) -> PLLSAI1SEC_R {
        PLLSAI1SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLLSAI2SEC"]
    #[inline(always)]
    pub fn pllsai2sec(&self) -> PLLSAI2SEC_R {
        PLLSAI2SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CLK48MSEC"]
    #[inline(always)]
    pub fn clk48msec(&self) -> CLK48MSEC_R {
        CLK48MSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI48SEC"]
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RMVFSEC"]
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSISEC"]
    #[inline(always)]
    #[must_use]
    pub fn hsisec(&mut self) -> HSISEC_W<0> {
        HSISEC_W::new(self)
    }
    #[doc = "Bit 1 - HSESEC"]
    #[inline(always)]
    #[must_use]
    pub fn hsesec(&mut self) -> HSESEC_W<1> {
        HSESEC_W::new(self)
    }
    #[doc = "Bit 2 - MSISEC"]
    #[inline(always)]
    #[must_use]
    pub fn msisec(&mut self) -> MSISEC_W<2> {
        MSISEC_W::new(self)
    }
    #[doc = "Bit 3 - LSISEC"]
    #[inline(always)]
    #[must_use]
    pub fn lsisec(&mut self) -> LSISEC_W<3> {
        LSISEC_W::new(self)
    }
    #[doc = "Bit 4 - LSESEC"]
    #[inline(always)]
    #[must_use]
    pub fn lsesec(&mut self) -> LSESEC_W<4> {
        LSESEC_W::new(self)
    }
    #[doc = "Bit 5 - SYSCLKSEC"]
    #[inline(always)]
    #[must_use]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W<5> {
        SYSCLKSEC_W::new(self)
    }
    #[doc = "Bit 6 - PRESCSEC"]
    #[inline(always)]
    #[must_use]
    pub fn prescsec(&mut self) -> PRESCSEC_W<6> {
        PRESCSEC_W::new(self)
    }
    #[doc = "Bit 7 - PLLSEC"]
    #[inline(always)]
    #[must_use]
    pub fn pllsec(&mut self) -> PLLSEC_W<7> {
        PLLSEC_W::new(self)
    }
    #[doc = "Bit 8 - PLLSAI1SEC"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1sec(&mut self) -> PLLSAI1SEC_W<8> {
        PLLSAI1SEC_W::new(self)
    }
    #[doc = "Bit 9 - PLLSAI2SEC"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2sec(&mut self) -> PLLSAI2SEC_W<9> {
        PLLSAI2SEC_W::new(self)
    }
    #[doc = "Bit 10 - CLK48MSEC"]
    #[inline(always)]
    #[must_use]
    pub fn clk48msec(&mut self) -> CLK48MSEC_W<10> {
        CLK48MSEC_W::new(self)
    }
    #[doc = "Bit 11 - HSI48SEC"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W<11> {
        HSI48SEC_W::new(self)
    }
    #[doc = "Bit 12 - RMVFSEC"]
    #[inline(always)]
    #[must_use]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W<12> {
        RMVFSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr::R](R) reader structure"]
impl crate::Readable for SECCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
