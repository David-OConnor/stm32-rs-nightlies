#[doc = "Register `AF1` reader"]
pub struct R(crate::R<AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AF1` writer"]
pub struct W(crate::W<AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF1_SPEC>;
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
impl From<crate::W<AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BKINE_R = crate::BitReader<bool>;
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BKINE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable"]
pub type BKCMP1E_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable"]
pub type BKCMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable"]
pub type BKCMP2E_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable"]
pub type BKCMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP3E` reader - BRK COMP3 enable"]
pub type BKCMP3E_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP3E` writer - BRK COMP3 enable"]
pub type BKCMP3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP4E` reader - BRK COMP4 enable"]
pub type BKCMP4E_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP4E` writer - BRK COMP4 enable"]
pub type BKCMP4E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP5E` reader - BRK COMP5 enable"]
pub type BKCMP5E_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP5E` writer - BRK COMP5 enable"]
pub type BKCMP5E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP6E` reader - BRK COMP6 enable"]
pub type BKCMP6E_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP6E` writer - BRK COMP6 enable"]
pub type BKCMP6E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP7E` reader - BRK COMP7 enable"]
pub type BKCMP7E_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP7E` writer - BRK COMP7 enable"]
pub type BKCMP7E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKINP` reader - BRK BKIN input polarity"]
pub type BKINP_R = crate::BitReader<bool>;
#[doc = "Field `BKINP` writer - BRK BKIN input polarity"]
pub type BKINP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity"]
pub type BKCMP1P_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity"]
pub type BKCMP1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarity"]
pub type BKCMP2P_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarity"]
pub type BKCMP2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP3P` reader - BRK COMP3 input polarity"]
pub type BKCMP3P_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP3P` writer - BRK COMP3 input polarity"]
pub type BKCMP3P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `BKCMP4P` reader - BRK COMP4 input polarity"]
pub type BKCMP4P_R = crate::BitReader<bool>;
#[doc = "Field `BKCMP4P` writer - BRK COMP4 input polarity"]
pub type BKCMP4P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
#[doc = "Field `ETRSEL` reader - ETR source selection"]
pub type ETRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETRSEL` writer - ETR source selection"]
pub type ETRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AF1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRK COMP3 enable"]
    #[inline(always)]
    pub fn bkcmp3e(&self) -> BKCMP3E_R {
        BKCMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BRK COMP4 enable"]
    #[inline(always)]
    pub fn bkcmp4e(&self) -> BKCMP4E_R {
        BKCMP4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRK COMP5 enable"]
    #[inline(always)]
    pub fn bkcmp5e(&self) -> BKCMP5E_R {
        BKCMP5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRK COMP6 enable"]
    #[inline(always)]
    pub fn bkcmp6e(&self) -> BKCMP6E_R {
        BKCMP6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BRK COMP7 enable"]
    #[inline(always)]
    pub fn bkcmp7e(&self) -> BKCMP7E_R {
        BKCMP7E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRK COMP3 input polarity"]
    #[inline(always)]
    pub fn bkcmp3p(&self) -> BKCMP3P_R {
        BKCMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BRK COMP4 input polarity"]
    #[inline(always)]
    pub fn bkcmp4p(&self) -> BKCMP4P_R {
        BKCMP4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - ETR source selection"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<0> {
        BKINE_W::new(self)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<1> {
        BKCMP1E_W::new(self)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<2> {
        BKCMP2E_W::new(self)
    }
    #[doc = "Bit 3 - BRK COMP3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp3e(&mut self) -> BKCMP3E_W<3> {
        BKCMP3E_W::new(self)
    }
    #[doc = "Bit 4 - BRK COMP4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp4e(&mut self) -> BKCMP4E_W<4> {
        BKCMP4E_W::new(self)
    }
    #[doc = "Bit 5 - BRK COMP5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp5e(&mut self) -> BKCMP5E_W<5> {
        BKCMP5E_W::new(self)
    }
    #[doc = "Bit 6 - BRK COMP6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp6e(&mut self) -> BKCMP6E_W<6> {
        BKCMP6E_W::new(self)
    }
    #[doc = "Bit 7 - BRK COMP7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp7e(&mut self) -> BKCMP7E_W<7> {
        BKCMP7E_W::new(self)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<9> {
        BKINP_W::new(self)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<10> {
        BKCMP1P_W::new(self)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<11> {
        BKCMP2P_W::new(self)
    }
    #[doc = "Bit 12 - BRK COMP3 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp3p(&mut self) -> BKCMP3P_W<12> {
        BKCMP3P_W::new(self)
    }
    #[doc = "Bit 13 - BRK COMP4 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp4p(&mut self) -> BKCMP4P_W<13> {
        BKCMP4P_W::new(self)
    }
    #[doc = "Bits 14:17 - ETR source selection"]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<14> {
        ETRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM alternate function option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af1](index.html) module"]
pub struct AF1_SPEC;
impl crate::RegisterSpec for AF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [af1::R](R) reader structure"]
impl crate::Readable for AF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [af1::W](W) writer structure"]
impl crate::Writable for AF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AF1 to value 0"]
impl crate::Resettable for AF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
