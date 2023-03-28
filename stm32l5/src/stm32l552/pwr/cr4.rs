#[doc = "Register `CR4` reader"]
pub struct R(crate::R<CR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR4` writer"]
pub struct W(crate::W<CR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR4_SPEC>;
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
impl From<crate::W<CR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUPP1` reader - Wakeup pin WKUP1 polarity"]
pub type WUPP1_R = crate::BitReader<bool>;
#[doc = "Field `WUPP1` writer - Wakeup pin WKUP1 polarity"]
pub type WUPP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `WUPP2` reader - Wakeup pin WKUP2 polarity"]
pub type WUPP2_R = crate::BitReader<bool>;
#[doc = "Field `WUPP2` writer - Wakeup pin WKUP2 polarity"]
pub type WUPP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `WUPP3` reader - Wakeup pin WKUP3 polarity"]
pub type WUPP3_R = crate::BitReader<bool>;
#[doc = "Field `WUPP3` writer - Wakeup pin WKUP3 polarity"]
pub type WUPP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `WUPP4` reader - Wakeup pin WKUP4 polarity"]
pub type WUPP4_R = crate::BitReader<bool>;
#[doc = "Field `WUPP4` writer - Wakeup pin WKUP4 polarity"]
pub type WUPP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `WUPP5` reader - Wakeup pin WKUP5 polarity"]
pub type WUPP5_R = crate::BitReader<bool>;
#[doc = "Field `WUPP5` writer - Wakeup pin WKUP5 polarity"]
pub type WUPP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `VBE` reader - VBAT battery charging enable"]
pub type VBE_R = crate::BitReader<bool>;
#[doc = "Field `VBE` writer - VBAT battery charging enable"]
pub type VBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `VBRS` reader - VBAT battery charging resistor selection"]
pub type VBRS_R = crate::BitReader<bool>;
#[doc = "Field `VBRS` writer - VBAT battery charging resistor selection"]
pub type VBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `SMPSBYP` reader - SMPSBYP"]
pub type SMPSBYP_R = crate::BitReader<bool>;
#[doc = "Field `SMPSBYP` writer - SMPSBYP"]
pub type SMPSBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `EXTSMPSEN` reader - EXTSMPSEN"]
pub type EXTSMPSEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTSMPSEN` writer - EXTSMPSEN"]
pub type EXTSMPSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `SMPSFSTEN` reader - SMPSFSTEN"]
pub type SMPSFSTEN_R = crate::BitReader<bool>;
#[doc = "Field `SMPSFSTEN` writer - SMPSFSTEN"]
pub type SMPSFSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `SMPSLPEN` reader - SMPSLPEN"]
pub type SMPSLPEN_R = crate::BitReader<bool>;
#[doc = "Field `SMPSLPEN` writer - SMPSLPEN"]
pub type SMPSLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity"]
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity"]
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SMPSBYP"]
    #[inline(always)]
    pub fn smpsbyp(&self) -> SMPSBYP_R {
        SMPSBYP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EXTSMPSEN"]
    #[inline(always)]
    pub fn extsmpsen(&self) -> EXTSMPSEN_R {
        EXTSMPSEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SMPSFSTEN"]
    #[inline(always)]
    pub fn smpsfsten(&self) -> SMPSFSTEN_R {
        SMPSFSTEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMPSLPEN"]
    #[inline(always)]
    pub fn smpslpen(&self) -> SMPSLPEN_R {
        SMPSLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wupp1(&mut self) -> WUPP1_W<0> {
        WUPP1_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wupp2(&mut self) -> WUPP2_W<1> {
        WUPP2_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wupp3(&mut self) -> WUPP3_W<2> {
        WUPP3_W::new(self)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wupp4(&mut self) -> WUPP4_W<3> {
        WUPP4_W::new(self)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wupp5(&mut self) -> WUPP5_W<4> {
        WUPP5_W::new(self)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<8> {
        VBE_W::new(self)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<9> {
        VBRS_W::new(self)
    }
    #[doc = "Bit 12 - SMPSBYP"]
    #[inline(always)]
    #[must_use]
    pub fn smpsbyp(&mut self) -> SMPSBYP_W<12> {
        SMPSBYP_W::new(self)
    }
    #[doc = "Bit 13 - EXTSMPSEN"]
    #[inline(always)]
    #[must_use]
    pub fn extsmpsen(&mut self) -> EXTSMPSEN_W<13> {
        EXTSMPSEN_W::new(self)
    }
    #[doc = "Bit 14 - SMPSFSTEN"]
    #[inline(always)]
    #[must_use]
    pub fn smpsfsten(&mut self) -> SMPSFSTEN_W<14> {
        SMPSFSTEN_W::new(self)
    }
    #[doc = "Bit 15 - SMPSLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn smpslpen(&mut self) -> SMPSLPEN_W<15> {
        SMPSLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr4](index.html) module"]
pub struct CR4_SPEC;
impl crate::RegisterSpec for CR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr4::R](R) reader structure"]
impl crate::Readable for CR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr4::W](W) writer structure"]
impl crate::Writable for CR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for CR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
