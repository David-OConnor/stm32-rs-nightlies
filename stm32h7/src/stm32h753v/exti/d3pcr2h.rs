#[doc = "Register `D3PCR2H` reader"]
pub struct R(crate::R<D3PCR2H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PCR2H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PCR2H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PCR2H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3PCR2H` writer"]
pub struct W(crate::W<D3PCR2H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PCR2H_SPEC>;
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
impl From<crate::W<D3PCR2H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PCR2H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCS48` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS48_R = crate::FieldReader<u8, PCS48_A>;
#[doc = "Pending request clear input signal selection on Event input x= truncate ((n+96)/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS48_A {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DmaCh6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DmaCh7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    Lptim4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    Lptim5 = 3,
}
impl From<PCS48_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS48_A) -> Self {
        variant as _
    }
}
impl PCS48_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS48_A {
        match self.bits {
            0 => PCS48_A::DmaCh6,
            1 => PCS48_A::DmaCh7,
            2 => PCS48_A::Lptim4,
            3 => PCS48_A::Lptim5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DmaCh6`"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS48_A::DmaCh6
    }
    #[doc = "Checks if the value of the field is `DmaCh7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS48_A::DmaCh7
    }
    #[doc = "Checks if the value of the field is `Lptim4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS48_A::Lptim4
    }
    #[doc = "Checks if the value of the field is `Lptim5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS48_A::Lptim5
    }
}
#[doc = "Field `PCS48` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS48_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, D3PCR2H_SPEC, u8, PCS48_A, 2, O>;
impl<'a, const O: u8> PCS48_W<'a, O> {
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS48_A::DmaCh6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS48_A::DmaCh7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS48_A::Lptim4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS48_A::Lptim5)
    }
}
#[doc = "Field `PCS49` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_R as PCS49_R;
#[doc = "Field `PCS50` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_R as PCS50_R;
#[doc = "Field `PCS51` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_R as PCS51_R;
#[doc = "Field `PCS52` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_R as PCS52_R;
#[doc = "Field `PCS53` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_R as PCS53_R;
#[doc = "Field `PCS49` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_W as PCS49_W;
#[doc = "Field `PCS50` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_W as PCS50_W;
#[doc = "Field `PCS51` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_W as PCS51_W;
#[doc = "Field `PCS52` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_W as PCS52_W;
#[doc = "Field `PCS53` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_W as PCS53_W;
impl R {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs48(&self) -> PCS48_R {
        PCS48_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs49(&self) -> PCS49_R {
        PCS49_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs50(&self) -> PCS50_R {
        PCS50_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs51(&self) -> PCS51_R {
        PCS51_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs52(&self) -> PCS52_R {
        PCS52_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs53(&self) -> PCS53_R {
        PCS53_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs48(&mut self) -> PCS48_W<0> {
        PCS48_W::new(self)
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs49(&mut self) -> PCS49_W<2> {
        PCS49_W::new(self)
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs50(&mut self) -> PCS50_W<4> {
        PCS50_W::new(self)
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs51(&mut self) -> PCS51_W<6> {
        PCS51_W::new(self)
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs52(&mut self) -> PCS52_W<8> {
        PCS52_W::new(self)
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs53(&mut self) -> PCS53_W<10> {
        PCS53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr2h](index.html) module"]
pub struct D3PCR2H_SPEC;
impl crate::RegisterSpec for D3PCR2H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3pcr2h::R](R) reader structure"]
impl crate::Readable for D3PCR2H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3pcr2h::W](W) writer structure"]
impl crate::Writable for D3PCR2H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3PCR2H to value 0"]
impl crate::Resettable for D3PCR2H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
