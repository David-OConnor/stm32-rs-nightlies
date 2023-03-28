#[doc = "Register `D3PCR1L` reader"]
pub struct R(crate::R<D3PCR1L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PCR1L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PCR1L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PCR1L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3PCR1L` writer"]
pub struct W(crate::W<D3PCR1L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PCR1L_SPEC>;
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
impl From<crate::W<D3PCR1L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PCR1L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCS0` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS0_R = crate::FieldReader<u8, PCS0_A>;
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS0_A {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DmaCh6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DmaCh7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    Lptim4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    Lptim5 = 3,
}
impl From<PCS0_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS0_A) -> Self {
        variant as _
    }
}
impl PCS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS0_A {
        match self.bits {
            0 => PCS0_A::DmaCh6,
            1 => PCS0_A::DmaCh7,
            2 => PCS0_A::Lptim4,
            3 => PCS0_A::Lptim5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DmaCh6`"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS0_A::DmaCh6
    }
    #[doc = "Checks if the value of the field is `DmaCh7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS0_A::DmaCh7
    }
    #[doc = "Checks if the value of the field is `Lptim4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS0_A::Lptim4
    }
    #[doc = "Checks if the value of the field is `Lptim5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS0_A::Lptim5
    }
}
#[doc = "Field `PCS0` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, D3PCR1L_SPEC, u8, PCS0_A, 2, O>;
impl<'a, const O: u8> PCS0_W<'a, O> {
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS0_A::DmaCh6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS0_A::DmaCh7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS0_A::Lptim4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS0_A::Lptim5)
    }
}
#[doc = "Field `PCS1` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS1_R;
#[doc = "Field `PCS2` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS2_R;
#[doc = "Field `PCS3` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS3_R;
#[doc = "Field `PCS4` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS4_R;
#[doc = "Field `PCS5` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS5_R;
#[doc = "Field `PCS6` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS6_R;
#[doc = "Field `PCS7` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS7_R;
#[doc = "Field `PCS8` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS8_R;
#[doc = "Field `PCS9` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS9_R;
#[doc = "Field `PCS10` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS10_R;
#[doc = "Field `PCS11` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS11_R;
#[doc = "Field `PCS12` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS12_R;
#[doc = "Field `PCS13` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS13_R;
#[doc = "Field `PCS14` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS14_R;
#[doc = "Field `PCS15` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS15_R;
#[doc = "Field `PCS1` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS1_W;
#[doc = "Field `PCS2` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS2_W;
#[doc = "Field `PCS3` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS3_W;
#[doc = "Field `PCS4` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS4_W;
#[doc = "Field `PCS5` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS5_W;
#[doc = "Field `PCS6` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS6_W;
#[doc = "Field `PCS7` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS7_W;
#[doc = "Field `PCS8` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS8_W;
#[doc = "Field `PCS9` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS9_W;
#[doc = "Field `PCS10` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS10_W;
#[doc = "Field `PCS11` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS11_W;
#[doc = "Field `PCS12` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS12_W;
#[doc = "Field `PCS13` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS13_W;
#[doc = "Field `PCS14` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS14_W;
#[doc = "Field `PCS15` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS15_W;
impl R {
    #[doc = "Bits 0:1 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs0(&self) -> PCS0_R {
        PCS0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs1(&self) -> PCS1_R {
        PCS1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs2(&self) -> PCS2_R {
        PCS2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs3(&self) -> PCS3_R {
        PCS3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs4(&self) -> PCS4_R {
        PCS4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs5(&self) -> PCS5_R {
        PCS5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs6(&self) -> PCS6_R {
        PCS6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs7(&self) -> PCS7_R {
        PCS7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs8(&self) -> PCS8_R {
        PCS8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs9(&self) -> PCS9_R {
        PCS9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs10(&self) -> PCS10_R {
        PCS10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs11(&self) -> PCS11_R {
        PCS11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs12(&self) -> PCS12_R {
        PCS12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs13(&self) -> PCS13_R {
        PCS13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs14(&self) -> PCS14_R {
        PCS14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs15(&self) -> PCS15_R {
        PCS15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs0(&mut self) -> PCS0_W<0> {
        PCS0_W::new(self)
    }
    #[doc = "Bits 2:3 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs1(&mut self) -> PCS1_W<2> {
        PCS1_W::new(self)
    }
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs2(&mut self) -> PCS2_W<4> {
        PCS2_W::new(self)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs3(&mut self) -> PCS3_W<6> {
        PCS3_W::new(self)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs4(&mut self) -> PCS4_W<8> {
        PCS4_W::new(self)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs5(&mut self) -> PCS5_W<10> {
        PCS5_W::new(self)
    }
    #[doc = "Bits 12:13 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs6(&mut self) -> PCS6_W<12> {
        PCS6_W::new(self)
    }
    #[doc = "Bits 14:15 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs7(&mut self) -> PCS7_W<14> {
        PCS7_W::new(self)
    }
    #[doc = "Bits 16:17 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs8(&mut self) -> PCS8_W<16> {
        PCS8_W::new(self)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs9(&mut self) -> PCS9_W<18> {
        PCS9_W::new(self)
    }
    #[doc = "Bits 20:21 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs10(&mut self) -> PCS10_W<20> {
        PCS10_W::new(self)
    }
    #[doc = "Bits 22:23 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs11(&mut self) -> PCS11_W<22> {
        PCS11_W::new(self)
    }
    #[doc = "Bits 24:25 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs12(&mut self) -> PCS12_W<24> {
        PCS12_W::new(self)
    }
    #[doc = "Bits 26:27 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs13(&mut self) -> PCS13_W<26> {
        PCS13_W::new(self)
    }
    #[doc = "Bits 28:29 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs14(&mut self) -> PCS14_W<28> {
        PCS14_W::new(self)
    }
    #[doc = "Bits 30:31 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs15(&mut self) -> PCS15_W<30> {
        PCS15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI D3 pending clear selection register low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr1l](index.html) module"]
pub struct D3PCR1L_SPEC;
impl crate::RegisterSpec for D3PCR1L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3pcr1l::R](R) reader structure"]
impl crate::Readable for D3PCR1L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3pcr1l::W](W) writer structure"]
impl crate::Writable for D3PCR1L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3PCR1L to value 0"]
impl crate::Resettable for D3PCR1L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
