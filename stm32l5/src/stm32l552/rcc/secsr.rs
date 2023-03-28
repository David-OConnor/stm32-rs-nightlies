#[doc = "Register `SECSR` reader"]
pub struct R(crate::R<SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECSR` writer"]
pub struct W(crate::W<SECSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECSR_SPEC>;
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
impl From<crate::W<SECSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSISECF` reader - HSISECF"]
pub type HSISECF_R = crate::BitReader<bool>;
#[doc = "Field `HSISECF` writer - HSISECF"]
pub type HSISECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `HSESECF` reader - HSESECF"]
pub type HSESECF_R = crate::BitReader<bool>;
#[doc = "Field `HSESECF` writer - HSESECF"]
pub type HSESECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `MSISECF` reader - MSISECF"]
pub type MSISECF_R = crate::BitReader<bool>;
#[doc = "Field `MSISECF` writer - MSISECF"]
pub type MSISECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `LSISECF` reader - LSISECF"]
pub type LSISECF_R = crate::BitReader<bool>;
#[doc = "Field `LSISECF` writer - LSISECF"]
pub type LSISECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `LSESECF` reader - LSESECF"]
pub type LSESECF_R = crate::BitReader<bool>;
#[doc = "Field `LSESECF` writer - LSESECF"]
pub type LSESECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `SYSCLKSECF` reader - SYSCLKSECF"]
pub type SYSCLKSECF_R = crate::BitReader<bool>;
#[doc = "Field `SYSCLKSECF` writer - SYSCLKSECF"]
pub type SYSCLKSECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `PRESCSECF` reader - PRESCSECF"]
pub type PRESCSECF_R = crate::BitReader<bool>;
#[doc = "Field `PRESCSECF` writer - PRESCSECF"]
pub type PRESCSECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `PLLSECF` reader - PLLSECF"]
pub type PLLSECF_R = crate::BitReader<bool>;
#[doc = "Field `PLLSECF` writer - PLLSECF"]
pub type PLLSECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `PLLSAI1SECF` reader - PLLSAI1SECF"]
pub type PLLSAI1SECF_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI1SECF` writer - PLLSAI1SECF"]
pub type PLLSAI1SECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `PLLSAI2SECF` reader - PLLSAI2SECF"]
pub type PLLSAI2SECF_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI2SECF` writer - PLLSAI2SECF"]
pub type PLLSAI2SECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `CLK48MSECF` reader - CLK48MSECF"]
pub type CLK48MSECF_R = crate::BitReader<bool>;
#[doc = "Field `CLK48MSECF` writer - CLK48MSECF"]
pub type CLK48MSECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `HSI48SECF` reader - HSI48SECF"]
pub type HSI48SECF_R = crate::BitReader<bool>;
#[doc = "Field `HSI48SECF` writer - HSI48SECF"]
pub type HSI48SECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
#[doc = "Field `RMVFSECF` reader - RMVFSECF"]
pub type RMVFSECF_R = crate::BitReader<bool>;
#[doc = "Field `RMVFSECF` writer - RMVFSECF"]
pub type RMVFSECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HSISECF"]
    #[inline(always)]
    pub fn hsisecf(&self) -> HSISECF_R {
        HSISECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSESECF"]
    #[inline(always)]
    pub fn hsesecf(&self) -> HSESECF_R {
        HSESECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSISECF"]
    #[inline(always)]
    pub fn msisecf(&self) -> MSISECF_R {
        MSISECF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSISECF"]
    #[inline(always)]
    pub fn lsisecf(&self) -> LSISECF_R {
        LSISECF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LSESECF"]
    #[inline(always)]
    pub fn lsesecf(&self) -> LSESECF_R {
        LSESECF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYSCLKSECF"]
    #[inline(always)]
    pub fn sysclksecf(&self) -> SYSCLKSECF_R {
        SYSCLKSECF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PRESCSECF"]
    #[inline(always)]
    pub fn prescsecf(&self) -> PRESCSECF_R {
        PRESCSECF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSECF"]
    #[inline(always)]
    pub fn pllsecf(&self) -> PLLSECF_R {
        PLLSECF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLLSAI1SECF"]
    #[inline(always)]
    pub fn pllsai1secf(&self) -> PLLSAI1SECF_R {
        PLLSAI1SECF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLLSAI2SECF"]
    #[inline(always)]
    pub fn pllsai2secf(&self) -> PLLSAI2SECF_R {
        PLLSAI2SECF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CLK48MSECF"]
    #[inline(always)]
    pub fn clk48msecf(&self) -> CLK48MSECF_R {
        CLK48MSECF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI48SECF"]
    #[inline(always)]
    pub fn hsi48secf(&self) -> HSI48SECF_R {
        HSI48SECF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RMVFSECF"]
    #[inline(always)]
    pub fn rmvfsecf(&self) -> RMVFSECF_R {
        RMVFSECF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSISECF"]
    #[inline(always)]
    #[must_use]
    pub fn hsisecf(&mut self) -> HSISECF_W<0> {
        HSISECF_W::new(self)
    }
    #[doc = "Bit 1 - HSESECF"]
    #[inline(always)]
    #[must_use]
    pub fn hsesecf(&mut self) -> HSESECF_W<1> {
        HSESECF_W::new(self)
    }
    #[doc = "Bit 2 - MSISECF"]
    #[inline(always)]
    #[must_use]
    pub fn msisecf(&mut self) -> MSISECF_W<2> {
        MSISECF_W::new(self)
    }
    #[doc = "Bit 3 - LSISECF"]
    #[inline(always)]
    #[must_use]
    pub fn lsisecf(&mut self) -> LSISECF_W<3> {
        LSISECF_W::new(self)
    }
    #[doc = "Bit 4 - LSESECF"]
    #[inline(always)]
    #[must_use]
    pub fn lsesecf(&mut self) -> LSESECF_W<4> {
        LSESECF_W::new(self)
    }
    #[doc = "Bit 5 - SYSCLKSECF"]
    #[inline(always)]
    #[must_use]
    pub fn sysclksecf(&mut self) -> SYSCLKSECF_W<5> {
        SYSCLKSECF_W::new(self)
    }
    #[doc = "Bit 6 - PRESCSECF"]
    #[inline(always)]
    #[must_use]
    pub fn prescsecf(&mut self) -> PRESCSECF_W<6> {
        PRESCSECF_W::new(self)
    }
    #[doc = "Bit 7 - PLLSECF"]
    #[inline(always)]
    #[must_use]
    pub fn pllsecf(&mut self) -> PLLSECF_W<7> {
        PLLSECF_W::new(self)
    }
    #[doc = "Bit 8 - PLLSAI1SECF"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1secf(&mut self) -> PLLSAI1SECF_W<8> {
        PLLSAI1SECF_W::new(self)
    }
    #[doc = "Bit 9 - PLLSAI2SECF"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2secf(&mut self) -> PLLSAI2SECF_W<9> {
        PLLSAI2SECF_W::new(self)
    }
    #[doc = "Bit 10 - CLK48MSECF"]
    #[inline(always)]
    #[must_use]
    pub fn clk48msecf(&mut self) -> CLK48MSECF_W<10> {
        CLK48MSECF_W::new(self)
    }
    #[doc = "Bit 11 - HSI48SECF"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48secf(&mut self) -> HSI48SECF_W<11> {
        HSI48SECF_W::new(self)
    }
    #[doc = "Bit 12 - RMVFSECF"]
    #[inline(always)]
    #[must_use]
    pub fn rmvfsecf(&mut self) -> RMVFSECF_W<12> {
        RMVFSECF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC secure status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secsr](index.html) module"]
pub struct SECSR_SPEC;
impl crate::RegisterSpec for SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secsr::R](R) reader structure"]
impl crate::Readable for SECSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secsr::W](W) writer structure"]
impl crate::Writable for SECSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECSR to value 0"]
impl crate::Resettable for SECSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
