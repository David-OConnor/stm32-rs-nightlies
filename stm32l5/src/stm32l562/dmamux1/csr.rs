#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOF0` reader - Synchronization Overrun Flag 0"]
pub type SOF0_R = crate::BitReader<bool>;
#[doc = "Field `SOF0` writer - Synchronization Overrun Flag 0"]
pub type SOF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF1` reader - Synchronization Overrun Flag 1"]
pub type SOF1_R = crate::BitReader<bool>;
#[doc = "Field `SOF1` writer - Synchronization Overrun Flag 1"]
pub type SOF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF2` reader - Synchronization Overrun Flag 2"]
pub type SOF2_R = crate::BitReader<bool>;
#[doc = "Field `SOF2` writer - Synchronization Overrun Flag 2"]
pub type SOF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF3` reader - Synchronization Overrun Flag 3"]
pub type SOF3_R = crate::BitReader<bool>;
#[doc = "Field `SOF3` writer - Synchronization Overrun Flag 3"]
pub type SOF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF4` reader - Synchronization Overrun Flag 4"]
pub type SOF4_R = crate::BitReader<bool>;
#[doc = "Field `SOF4` writer - Synchronization Overrun Flag 4"]
pub type SOF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF5` reader - Synchronization Overrun Flag 5"]
pub type SOF5_R = crate::BitReader<bool>;
#[doc = "Field `SOF5` writer - Synchronization Overrun Flag 5"]
pub type SOF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF6` reader - Synchronization Overrun Flag 6"]
pub type SOF6_R = crate::BitReader<bool>;
#[doc = "Field `SOF6` writer - Synchronization Overrun Flag 6"]
pub type SOF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF7` reader - Synchronization Overrun Flag 7"]
pub type SOF7_R = crate::BitReader<bool>;
#[doc = "Field `SOF7` writer - Synchronization Overrun Flag 7"]
pub type SOF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF8` reader - Synchronization Overrun Flag 8"]
pub type SOF8_R = crate::BitReader<bool>;
#[doc = "Field `SOF8` writer - Synchronization Overrun Flag 8"]
pub type SOF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF9` reader - Synchronization Overrun Flag 9"]
pub type SOF9_R = crate::BitReader<bool>;
#[doc = "Field `SOF9` writer - Synchronization Overrun Flag 9"]
pub type SOF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF10` reader - Synchronization Overrun Flag 10"]
pub type SOF10_R = crate::BitReader<bool>;
#[doc = "Field `SOF10` writer - Synchronization Overrun Flag 10"]
pub type SOF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF11` reader - Synchronization Overrun Flag 11"]
pub type SOF11_R = crate::BitReader<bool>;
#[doc = "Field `SOF11` writer - Synchronization Overrun Flag 11"]
pub type SOF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF12` reader - Synchronization Overrun Flag 12"]
pub type SOF12_R = crate::BitReader<bool>;
#[doc = "Field `SOF12` writer - Synchronization Overrun Flag 12"]
pub type SOF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF13` reader - Synchronization Overrun Flag 13"]
pub type SOF13_R = crate::BitReader<bool>;
#[doc = "Field `SOF13` writer - Synchronization Overrun Flag 13"]
pub type SOF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF14` reader - Synchronization Overrun Flag 13"]
pub type SOF14_R = crate::BitReader<bool>;
#[doc = "Field `SOF14` writer - Synchronization Overrun Flag 13"]
pub type SOF14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SOF15` reader - Synchronization Overrun Flag 13"]
pub type SOF15_R = crate::BitReader<bool>;
#[doc = "Field `SOF15` writer - Synchronization Overrun Flag 13"]
pub type SOF15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Synchronization Overrun Flag 0"]
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization Overrun Flag 1"]
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization Overrun Flag 2"]
    #[inline(always)]
    pub fn sof2(&self) -> SOF2_R {
        SOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Overrun Flag 3"]
    #[inline(always)]
    pub fn sof3(&self) -> SOF3_R {
        SOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization Overrun Flag 4"]
    #[inline(always)]
    pub fn sof4(&self) -> SOF4_R {
        SOF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization Overrun Flag 5"]
    #[inline(always)]
    pub fn sof5(&self) -> SOF5_R {
        SOF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization Overrun Flag 6"]
    #[inline(always)]
    pub fn sof6(&self) -> SOF6_R {
        SOF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization Overrun Flag 7"]
    #[inline(always)]
    pub fn sof7(&self) -> SOF7_R {
        SOF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronization Overrun Flag 8"]
    #[inline(always)]
    pub fn sof8(&self) -> SOF8_R {
        SOF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Synchronization Overrun Flag 9"]
    #[inline(always)]
    pub fn sof9(&self) -> SOF9_R {
        SOF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization Overrun Flag 10"]
    #[inline(always)]
    pub fn sof10(&self) -> SOF10_R {
        SOF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Overrun Flag 11"]
    #[inline(always)]
    pub fn sof11(&self) -> SOF11_R {
        SOF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Synchronization Overrun Flag 12"]
    #[inline(always)]
    pub fn sof12(&self) -> SOF12_R {
        SOF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    pub fn sof13(&self) -> SOF13_R {
        SOF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    pub fn sof14(&self) -> SOF14_R {
        SOF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    pub fn sof15(&self) -> SOF15_R {
        SOF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronization Overrun Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn sof0(&mut self) -> SOF0_W<0> {
        SOF0_W::new(self)
    }
    #[doc = "Bit 1 - Synchronization Overrun Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn sof1(&mut self) -> SOF1_W<1> {
        SOF1_W::new(self)
    }
    #[doc = "Bit 2 - Synchronization Overrun Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn sof2(&mut self) -> SOF2_W<2> {
        SOF2_W::new(self)
    }
    #[doc = "Bit 3 - Synchronization Overrun Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn sof3(&mut self) -> SOF3_W<3> {
        SOF3_W::new(self)
    }
    #[doc = "Bit 4 - Synchronization Overrun Flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn sof4(&mut self) -> SOF4_W<4> {
        SOF4_W::new(self)
    }
    #[doc = "Bit 5 - Synchronization Overrun Flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn sof5(&mut self) -> SOF5_W<5> {
        SOF5_W::new(self)
    }
    #[doc = "Bit 6 - Synchronization Overrun Flag 6"]
    #[inline(always)]
    #[must_use]
    pub fn sof6(&mut self) -> SOF6_W<6> {
        SOF6_W::new(self)
    }
    #[doc = "Bit 7 - Synchronization Overrun Flag 7"]
    #[inline(always)]
    #[must_use]
    pub fn sof7(&mut self) -> SOF7_W<7> {
        SOF7_W::new(self)
    }
    #[doc = "Bit 8 - Synchronization Overrun Flag 8"]
    #[inline(always)]
    #[must_use]
    pub fn sof8(&mut self) -> SOF8_W<8> {
        SOF8_W::new(self)
    }
    #[doc = "Bit 9 - Synchronization Overrun Flag 9"]
    #[inline(always)]
    #[must_use]
    pub fn sof9(&mut self) -> SOF9_W<9> {
        SOF9_W::new(self)
    }
    #[doc = "Bit 10 - Synchronization Overrun Flag 10"]
    #[inline(always)]
    #[must_use]
    pub fn sof10(&mut self) -> SOF10_W<10> {
        SOF10_W::new(self)
    }
    #[doc = "Bit 11 - Synchronization Overrun Flag 11"]
    #[inline(always)]
    #[must_use]
    pub fn sof11(&mut self) -> SOF11_W<11> {
        SOF11_W::new(self)
    }
    #[doc = "Bit 12 - Synchronization Overrun Flag 12"]
    #[inline(always)]
    #[must_use]
    pub fn sof12(&mut self) -> SOF12_W<12> {
        SOF12_W::new(self)
    }
    #[doc = "Bit 13 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    #[must_use]
    pub fn sof13(&mut self) -> SOF13_W<13> {
        SOF13_W::new(self)
    }
    #[doc = "Bit 14 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    #[must_use]
    pub fn sof14(&mut self) -> SOF14_W<14> {
        SOF14_W::new(self)
    }
    #[doc = "Bit 15 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    #[must_use]
    pub fn sof15(&mut self) -> SOF15_W<15> {
        SOF15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Multiplexer Channel Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
