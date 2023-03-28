#[doc = "Register `AF2` reader"]
pub struct R(crate::R<AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AF2` writer"]
pub struct W(crate::W<AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF2_SPEC>;
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
impl From<crate::W<AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BKINE_R = crate::BitReader<bool>;
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BKINE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable"]
pub type BK2CMP1E_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable"]
pub type BK2CMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable"]
pub type BK2CMP2E_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable"]
pub type BK2CMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP3E` reader - BRK2 COMP3 enable"]
pub type BK2CMP3E_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP3E` writer - BRK2 COMP3 enable"]
pub type BK2CMP3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP4E` reader - BRK2 COMP4 enable"]
pub type BK2CMP4E_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP4E` writer - BRK2 COMP4 enable"]
pub type BK2CMP4E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP5E` reader - BRK2 COMP5 enable"]
pub type BK2CMP5E_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP5E` writer - BRK2 COMP5 enable"]
pub type BK2CMP5E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP6E` reader - BRK2 COMP6 enable"]
pub type BK2CMP6E_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP6E` writer - BRK2 COMP6 enable"]
pub type BK2CMP6E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP7E` reader - BRK2 COMP7 enable"]
pub type BK2CMP7E_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP7E` writer - BRK2 COMP7 enable"]
pub type BK2CMP7E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2INP` reader - BRK2 BKIN input polarity"]
pub type BK2INP_R = crate::BitReader<bool>;
#[doc = "Field `BK2INP` writer - BRK2 BKIN input polarity"]
pub type BK2INP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity"]
pub type BK2CMP1P_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity"]
pub type BK2CMP1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity"]
pub type BK2CMP2P_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity"]
pub type BK2CMP2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP3P` reader - BRK2 COMP3 input polarity"]
pub type BK2CMP3P_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP3P` writer - BRK2 COMP3 input polarity"]
pub type BK2CMP3P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `BK2CMP4P` reader - BRK2 COMP4 input polarity"]
pub type BK2CMP4P_R = crate::BitReader<bool>;
#[doc = "Field `BK2CMP4P` writer - BRK2 COMP4 input polarity"]
pub type BK2CMP4P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF2_SPEC, bool, O>;
#[doc = "Field `OCRSEL` reader - OCREF_CLR source selection"]
pub type OCRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCRSEL` writer - OCREF_CLR source selection"]
pub type OCRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AF2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRK2 COMP3 enable"]
    #[inline(always)]
    pub fn bk2cmp3e(&self) -> BK2CMP3E_R {
        BK2CMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BRK2 COMP4 enable"]
    #[inline(always)]
    pub fn bk2cmp4e(&self) -> BK2CMP4E_R {
        BK2CMP4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRK2 COMP5 enable"]
    #[inline(always)]
    pub fn bk2cmp5e(&self) -> BK2CMP5E_R {
        BK2CMP5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRK2 COMP6 enable"]
    #[inline(always)]
    pub fn bk2cmp6e(&self) -> BK2CMP6E_R {
        BK2CMP6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BRK2 COMP7 enable"]
    #[inline(always)]
    pub fn bk2cmp7e(&self) -> BK2CMP7E_R {
        BK2CMP7E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRK2 COMP3 input polarity"]
    #[inline(always)]
    pub fn bk2cmp3p(&self) -> BK2CMP3P_R {
        BK2CMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BRK2 COMP4 input polarity"]
    #[inline(always)]
    pub fn bk2cmp4p(&self) -> BK2CMP4P_R {
        BK2CMP4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18 - OCREF_CLR source selection"]
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<0> {
        BKINE_W::new(self)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<1> {
        BK2CMP1E_W::new(self)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<2> {
        BK2CMP2E_W::new(self)
    }
    #[doc = "Bit 3 - BRK2 COMP3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp3e(&mut self) -> BK2CMP3E_W<3> {
        BK2CMP3E_W::new(self)
    }
    #[doc = "Bit 4 - BRK2 COMP4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp4e(&mut self) -> BK2CMP4E_W<4> {
        BK2CMP4E_W::new(self)
    }
    #[doc = "Bit 5 - BRK2 COMP5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp5e(&mut self) -> BK2CMP5E_W<5> {
        BK2CMP5E_W::new(self)
    }
    #[doc = "Bit 6 - BRK2 COMP6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp6e(&mut self) -> BK2CMP6E_W<6> {
        BK2CMP6E_W::new(self)
    }
    #[doc = "Bit 7 - BRK2 COMP7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp7e(&mut self) -> BK2CMP7E_W<7> {
        BK2CMP7E_W::new(self)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<9> {
        BK2INP_W::new(self)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<10> {
        BK2CMP1P_W::new(self)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<11> {
        BK2CMP2P_W::new(self)
    }
    #[doc = "Bit 12 - BRK2 COMP3 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp3p(&mut self) -> BK2CMP3P_W<12> {
        BK2CMP3P_W::new(self)
    }
    #[doc = "Bit 13 - BRK2 COMP4 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp4p(&mut self) -> BK2CMP4P_W<13> {
        BK2CMP4P_W::new(self)
    }
    #[doc = "Bits 16:18 - OCREF_CLR source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ocrsel(&mut self) -> OCRSEL_W<16> {
        OCRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM alternate function option register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af2](index.html) module"]
pub struct AF2_SPEC;
impl crate::RegisterSpec for AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [af2::R](R) reader structure"]
impl crate::Readable for AF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [af2::W](W) writer structure"]
impl crate::Writable for AF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AF2 to value 0"]
impl crate::Resettable for AF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
